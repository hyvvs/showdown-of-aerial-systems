# Origin: owner-provided verifier at target/verifier/scratch/independent.py.
# Preserved executable body verbatim; only this provenance header was added.
# Independent informational reference: NOT part of the build or CI.
# Imports no repository code; uses the existing verifier NumPy environment.
# Do not alter its logic to fit repository results.
"""Independent verification: NumPy linear algebra, no repository imports or code.

Body x is forward, y right, z down. Unit speed and density cancel in the
coefficients. Linearized oncoming velocity=(-1,0,-alpha). Vortex A->B has
positive y orientation; wake is at -infinity in x. All inputs synthetic.
Sources: MIT Fluids Lecture 9 (Fourier LLT), Drela JVL (Biot-Savart/VLM).
"""
import json
import sys
import time
from pathlib import Path
import numpy as np

OUT = Path(__file__).parent
PI = np.pi
ALPHA = 0.05


def lifting_line(ar, terms, alpha=ALPHA, ellipse=False):
    # Gamma=2*b*V*sum(A_n*sin(n*theta)); use odd modes and half-span symmetry.
    theta = np.arange(1, terms + 1) * PI / (2 * terms)
    n = np.arange(1, 2 * terms, 2)
    span = ar if not ellipse else ar * PI / 4
    chord = np.ones(terms) if not ellipse else np.sin(theta)
    sines = np.sin(theta[:, None] * n)
    matrix = sines * (4 * span / (2 * PI * chord[:, None])
                      + n / np.sin(theta[:, None]))
    a = np.linalg.solve(matrix, np.full(terms, alpha))
    cl = PI * ar * a[0]
    cd = PI * ar * np.dot(n, a * a)
    return dict(cl=float(cl), cdi=float(cd), e=float(cl * cl / (PI * ar * cd)))


def make_mesh(ar, ny, nx, ellipse):
    span = ar * PI / 4 if ellipse else float(ar)
    edge_y = -span / 2 * np.cos(np.linspace(0, PI, ny + 1))
    edge_y[ny // 2] = 0.0
    edge_c = (np.sqrt(np.maximum(0.0, 1 - (2 * edge_y / span)**2))
              if ellipse else np.ones(ny + 1))
    if ellipse:
        edge_c = np.maximum(edge_c, np.finfo(float).eps)
        edge_c *= (span * span / ar) / np.trapezoid(edge_c, edge_y)
    edge_le = edge_c / 4 if ellipse else np.zeros(ny + 1)
    # Flatten in strip-major order, chord index changing fastest.
    ya = np.repeat(edge_y[:-1], nx)
    yb = np.repeat(edge_y[1:], nx)
    ca = np.repeat(edge_c[:-1], nx)
    cb = np.repeat(edge_c[1:], nx)
    lea = np.repeat(edge_le[:-1], nx)
    leb = np.repeat(edge_le[1:], nx)
    f_bound = np.tile((np.arange(nx) + 0.25) / nx, ny)
    f_control = np.tile((np.arange(nx) + 0.75) / nx, ny)
    xa, xb = lea - f_bound * ca, leb - f_bound * cb
    xc = (lea + leb - f_control * (ca + cb)) / 2
    yc = (ya + yb) / 2
    area = float(np.trapezoid(edge_c, edge_y))
    return xa, ya, xb, yb, xc, yc, edge_y, area


def horseshoe_w(px, py, xa, ya, xb, yb):
    # Analytic line-integral of Biot-Savart over A->B and two semi-infinite rays.
    ax, ay = px[:, None] - xa, py[:, None] - ya
    bx, by = px[:, None] - xb, py[:, None] - yb
    ra, rb = np.hypot(ax, ay), np.hypot(bx, by)
    cross = ax * by - ay * bx
    numerator = (xb - xa) * (ax / ra - bx / rb) + (yb - ya) * (ay / ra - by / rb)
    finite = np.zeros_like(cross)
    np.divide(numerator, cross, out=finite, where=np.abs(cross) > 1e-25)
    # Collinear bound-segment self velocity is excluded in the KJ principal value.
    rays = (1 - ax / ra) / ay - (1 - bx / rb) / by
    return (finite + rays) / (4 * PI)


def vlm(ar, ny, nx=8, ellipse=False, kj=False):
    start = time.perf_counter()
    xa, ya, xb, yb, xc, yc, edges, area = make_mesh(ar, ny, nx, ellipse)
    # Mirror symmetry halves the unknown count. Each column includes both
    # right-half source and its left-half mirror, preserving positive-y orientation.
    half = ny * nx // 2
    xa, ya, xb, yb = xa[half:], ya[half:], xb[half:], yb[half:]
    xc, yc = xc[half:], yc[half:]
    def influence(px, py):
        return (horseshoe_w(px, py, xa, ya, xb, yb)
                + horseshoe_w(px, py, xb, -yb, xa, -ya))
    matrix = np.empty((half, half))
    block = 64
    for i in range(0, half, block):
        matrix[i:i+block] = influence(xc[i:i+block], yc[i:i+block])
    gamma = np.linalg.solve(matrix, np.full(half, ALPHA))
    residual = float(np.max(np.abs(matrix @ gamma - ALPHA)))
    del matrix
    right_g = gamma.reshape(ny // 2, nx).sum(axis=1)
    strip_g = np.concatenate((right_g[::-1], right_g))
    mid = (edges[1:] + edges[:-1]) / 2
    dy = np.diff(edges)
    # Downstream-infinity limit of the two rays. Farfield drag has factor 1/2
    # relative to rho*Gamma*w, because farfield downwash doubles wing downwash.
    far = (1 / (mid[:, None] - edges[:-1])
           - 1 / (mid[:, None] - edges[1:])) @ strip_g / (2 * PI)
    cl = 2 * np.dot(strip_g, dy) / area
    cd = np.dot(strip_g * far, dy) / area
    out = dict(ar=ar, ny=ny, nx=nx, ellipse=ellipse, cl=float(cl), cdi=float(cd),
               e=float(cl * cl / (PI * ar * cd)), residual=residual,
               max_panel_width=float(dy.max()), seconds=time.perf_counter()-start)
    if kj:
        # Kutta-Joukowski at bound midpoints: rho*Gamma*(V cross dl).
        # Projecting onto the linearized wind direction cancels alpha*lift,
        # leaving sum rho*Gamma*w_bound*dy. Include own rays, exclude own bound.
        bound_x, bound_y = (xa + xb) / 2, (ya + yb) / 2
        down = np.empty(half)
        for i in range(0, half, block):
            down[i:i+block] = influence(bound_x[i:i+block], bound_y[i:i+block]) @ gamma
        kj_cd = 4 * np.dot(gamma * down, yb - ya) / area
        out.update(kj_cdi=float(kj_cd), kj_minus_trefftz=float(kj_cd-cd),
                   kj_relative_difference=float((kj_cd-cd)/cd))
    return out


def emit(kind, result):
    print(kind, json.dumps(result), flush=True)
    with (OUT / 'independent_results.jsonl').open('a') as f:
        f.write(json.dumps(dict(kind=kind, **result)) + '\n')


def richardson(f):
    d0, d1 = f[1]-f[0], f[2]-f[1]
    ratio = d0/d1
    return dict(p=float(np.log2(ratio)), limit=float(f[2]+d1/(ratio-1)))


def main(mode):
    if mode == 'reproduce':
        for ar in (40, 10):
            ll = lifting_line(ar, 128)
            emit('LL128', dict(ar=ar, **ll))
            for ny in (64, 512):
                v = vlm(ar, ny, kj=(ar == 40))
                v.update(cl_gap=abs(v['cl']/ll['cl']-1), cdi_gap=abs(v['cdi']/ll['cdi']-1))
                emit('reproduce', v)
    elif mode == 'sweep':
        rows = []
        for ar, ny in ((10,64),(20,128),(40,256),(80,512),(160,1024)):
            refs = []
            for terms in (128, 256, 512, 1024):
                ll = lifting_line(ar, terms)
                emit('LL_convergence', dict(ar=ar, terms=terms, **ll))
                refs.append(ll)
            v = vlm(ar, ny)
            ll = refs[-1]
            v.update(cl_gap=abs(v['cl']/ll['cl']-1), cdi_gap=abs(v['cdi']/ll['cdi']-1))
            emit('sweep', v)
            rows.append(v)
        for quantity in ('cl_gap', 'cdi_gap'):
            slope, intercept = np.polyfit(np.log([r['ar'] for r in rows[1:]]),
                                         np.log([r[quantity] for r in rows[1:]]), 1)
            emit('power_fit', dict(quantity=quantity, p=float(-slope), scale=float(np.exp(intercept))))
    elif mode == 'ellipse':
        for nx in (1, 8):
            for ar in (6, 40):
                values = []
                for ny in (32, 64, 128, 256, 512, 1024):
                    v = vlm(ar, ny, nx=nx, ellipse=True)
                    target = 2 * PI / (1 + 2/ar)
                    v['relative_slope_gap'] = abs(v['cl']/ALPHA/target-1)
                    emit('ellipse', v)
                    values.append(v['e'])
                for k in range(len(values)-2):
                    emit('ellipse_R', dict(ar=ar, nx=nx, meshes=[32*2**j for j in range(k,k+3)],
                                           **richardson(values[k:k+3])))
    else:
        raise ValueError(mode)


if __name__ == '__main__':
    main(sys.argv[1])
