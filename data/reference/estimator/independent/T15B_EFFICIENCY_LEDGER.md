# T15b efficiency experiment: complete numerical ledger

All numeric paths below refer to the independent no-assertion experiment in vlm_efficiency_gap.md. CONFIG values are labeled SYNTHETIC TEST VALUES/settings. All other numbers are derived model observations or algebraic decompositions from those synthetic inputs, not real-aircraft data or pass/fail tolerances. Input terms/meshes were fixed before the run. The actual tip panel counts are computed by the stated insertion/subdivision algorithm.

| Result/configuration JSON path | Value | Provenance |
|---|---|---|
| `/config/cases/0/0` | `10` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/0/1` | `128` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/1/0` | `20` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/1/1` | `256` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/2/0` | `40` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/2/1` | `512` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/3/0` | `80` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/3/1` | `1024` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/4/0` | `160` | SYNTHETIC configured input/mesh/term setting |
| `/config/cases/4/1` | `2048` | SYNTHETIC configured input/mesh/term setting |
| `/config/nx` | `8` | SYNTHETIC configured input/mesh/term setting |
| `/config/alpha_rad` | `0.05` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/0` | `128` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/1` | `256` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/2` | `512` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/3` | `1024` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/4` | `2048` | SYNTHETIC configured input/mesh/term setting |
| `/config/fourier_terms/5` | `4096` | SYNTHETIC configured input/mesh/term setting |
| `/config/tip_ar/0` | `20` | SYNTHETIC configured input/mesh/term setting |
| `/config/tip_ar/1` | `40` | SYNTHETIC configured input/mesh/term setting |
| `/config/tip_semispan_fraction` | `0.05` | SYNTHETIC configured input/mesh/term setting |
| `/config/tip_subdivisions/0` | `2` | SYNTHETIC configured input/mesh/term setting |
| `/config/tip_subdivisions/1` | `4` | SYNTHETIC configured input/mesh/term setting |
| `/llt_convergence/0/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/0/terms` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/0/cl` | `0.252339542480009` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/0/cdi` | `0.0022009661869658143` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/0/e` | `0.920889199157196` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/terms` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/cl` | `0.2523395426930114` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/cdi` | `0.0022009661983049054` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/e` | `0.920889195967559` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/relative_cl_last_change` | `8.441103371836789e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/relative_cdi_last_change` | `5.1518698196417745e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/1/relative_e_last_change` | `3.4636490342521142e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/terms` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/cl` | `0.2523395427063158` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/cdi` | `0.0022009661990128144` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/e` | `0.9208891957684746` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/relative_cl_last_change` | `5.272426939484376e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/relative_cdi_last_change` | `3.216356070367965e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/2/relative_e_last_change` | `2.1618717926941144e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/terms` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/cl` | `0.25233954270714726` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/cdi` | `0.002200966199057046` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/e` | `0.9208891957560366` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/relative_cl_last_change` | `3.2949198924825396e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/relative_cdi_last_change` | `2.0096369013344884e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/3/relative_e_last_change` | `1.3506529228379804e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/terms` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/cl` | `0.2523395427071992` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/cdi` | `0.0022009661990598085` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/e` | `0.92088919575526` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/relative_cl_last_change` | `2.0583534876550402e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/relative_cdi_last_change` | `1.255218151641202e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/4/relative_e_last_change` | `8.433254095052689e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/terms` | `4096` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/cl` | `0.25233954270720266` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/cdi` | `0.0022009661990599855` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/e` | `0.9208891957552111` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/relative_cl_last_change` | `1.354472090042691e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/relative_cdi_last_change` | `8.038014698286133e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/5/relative_e_last_change` | `5.306866057708248e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/6/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/6/terms` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/6/cl` | `0.27721368294103704` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/6/cdi` | `0.0014208764493098343` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/6/e` | `0.8607819292999503` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/terms` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/cl` | `0.2772136833806799` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/cdi` | `0.001420876471968813` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/e` | `0.8607819183031907` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/relative_cl_last_change` | `1.5859349389302224e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/relative_cdi_last_change` | `1.5947184239806234e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/7/relative_e_last_change` | `1.2775314250923486e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/terms` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/cl` | `0.2772136834081275` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/cdi` | `0.0014208764733821427` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/e` | `0.860781917617437` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/relative_cl_last_change` | `9.901235387133056e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/relative_cdi_last_change` | `9.946887757905643e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/8/relative_e_last_change` | `7.966636239586933e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/terms` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/cl` | `0.27721368340984187` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/cdi` | `0.0014208764734704158` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/e` | `0.8607819175746066` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/relative_cl_last_change` | `6.184164291767047e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/relative_cdi_last_change` | `6.212586001197451e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/9/relative_e_last_change` | `4.9757531428440416e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/terms` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/cl` | `0.2772136834099493` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/cdi` | `0.0014208764734759344` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/e` | `0.8607819175719306` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/relative_cl_last_change` | `3.8746783559417963e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/relative_cdi_last_change` | `3.884004229348648e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/10/relative_e_last_change` | `3.1088465135553633e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/terms` | `4096` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/cl` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/cdi` | `0.0014208764734762762` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/e` | `0.8607819175717625` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/relative_cl_last_change` | `2.2648549702353193e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/relative_cdi_last_change` | `2.404743071338089e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/11/relative_e_last_change` | `1.9528823003156504e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/12/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/12/terms` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/12/cl` | `0.29278072785693654` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/12/cdi` | `0.0008624499010484641` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/12/e` | `0.7909357965772625` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/terms` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/cl` | `0.29278072875001054` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/cdi` | `0.0008624499464272138` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/e` | `0.7909357597865176` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/relative_cl_last_change` | `3.0503168879647546e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/relative_cdi_last_change` | `5.261609947737611e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/13/relative_e_last_change` | `4.6515463147933644e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/terms` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/cl` | `0.2927807288057169` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/cdi` | `0.0008624499492530854` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/e` | `0.790935757495944` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/relative_cl_last_change` | `1.9026646924658053e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/relative_cdi_last_change` | `3.2765630209041774e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/14/relative_e_last_change` | `2.8960298603664114e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/terms` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/cl` | `0.29278072880919637` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/cdi` | `0.0008624499494295042` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/e` | `0.790935757352953` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/relative_cl_last_change` | `1.18840493001926e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/relative_cdi_last_change` | `2.0455548366271614e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/15/relative_e_last_change` | `1.8078716301772602e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/terms` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/cl` | `0.2927807288094139` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/cdi` | `0.0008624499494405278` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/e` | `0.7909357573440189` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/relative_cl_last_change` | `7.429612480791548e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/relative_cdi_last_change` | `1.2781553593299577e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/16/relative_e_last_change` | `1.1295631097141268e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/terms` | `4096` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/cl` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/cdi` | `0.0008624499494412176` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/e` | `0.7909357573434621` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/relative_cl_last_change` | `4.796163466380676e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/relative_cdi_last_change` | `7.998046669399628e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/17/relative_e_last_change` | `7.039924199148118e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/18/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/18/terms` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/18/cl` | `0.3020547256263144` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/18/cdi` | `0.0005041100209165644` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/18/e` | `0.7201219999448095` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/terms` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/cl` | `0.30205472742867584` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/cdi` | `0.0005041101120176109` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/e` | `0.7201218784007758` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/relative_cl_last_change` | `5.967002891793527e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/relative_cdi_last_change` | `1.807165950218348e-07` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/19/relative_e_last_change` | `1.6878255870711456e-07` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/terms` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/cl` | `0.3020547275409246` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/cdi` | `0.0005041101176748251` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/e` | `0.7201218708546585` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/relative_cl_last_change` | `3.716174035162112e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/relative_cdi_last_change` | `1.1222179407965882e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/20/relative_e_last_change` | `1.047894460093346e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/terms` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/cl` | `0.3020547275479325` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/cdi` | `0.0005041101180276837` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/e` | `0.7201218703840143` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/relative_cl_last_change` | `2.3200774634801746e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/relative_cdi_last_change` | `6.999631985138421e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/21/relative_e_last_change` | `6.535618712888436e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/terms` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/cl` | `0.30205472754836993` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/cdi` | `0.0005041101180497226` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/e` | `0.7201218703546174` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/relative_cl_last_change` | `1.4481749133210542e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/relative_cdi_last_change` | `4.3718362263689414e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/22/relative_e_last_change` | `4.082212345934977e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/terms` | `4096` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/cl` | `0.30205472754839835` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/cdi` | `0.0005041101180511026` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/e` | `0.7201218703527816` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/relative_cl_last_change` | `9.414691248821327e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/relative_cdi_last_change` | `2.737587934120711e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/23/relative_e_last_change` | `2.549183086841822e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/24/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/24/terms` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/24/cl` | `0.30741088293810287` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/24/cdi` | `0.00028749366244517455` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/24/e` | `0.6539436782681894` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/terms` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/cl` | `0.3074108865690276` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/cdi` | `0.0002874938459296216` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/e` | `0.6539432763558225` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/relative_cl_last_change` | `1.181130837579758e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/relative_cdi_last_change` | `6.382208410649071e-07` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/25/relative_e_last_change` | `6.145978320715173e-07` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/terms` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/cl` | `0.30741088679454787` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/cdi` | `0.000287493857269346` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/e` | `0.653943251521582` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/relative_cl_last_change` | `7.336118379441814e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/relative_cdi_last_change` | `3.944336390837577e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/26/relative_e_last_change` | `3.7976138678175175e-08` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/terms` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/cl` | `0.307410886808615` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/cdi` | `0.0002874938579754997` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/e` | `0.6539432499751897` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/relative_cl_last_change` | `4.576006240597508e-11` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/relative_cdi_last_change` | `2.4562394340676974e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/27/relative_e_last_change` | `2.3647193092557472e-09` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/terms` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/cl` | `0.3074108868094929` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/cdi` | `0.0002874938580195837` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/e` | `0.6539432498786499` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/relative_cl_last_change` | `2.8557156639408277e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/relative_cdi_last_change` | `1.5333889713531335e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/28/relative_e_last_change` | `1.476271327405243e-10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/terms` | `4096` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/cl` | `0.30741088680954853` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/cdi` | `0.00028749385802233904` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/e` | `0.6539432498726192` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/relative_cl_last_change` | `1.8096635301390052e-13` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/relative_cdi_last_change` | `9.583889237774201e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/llt_convergence/29/relative_e_last_change` | `9.222067554048863e-12` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/ar` | `10` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/ny` | `128` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cl_vlm` | `0.24278151248986282` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cdi_vlm` | `0.0019403580727142072` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/e_vlm` | `0.966939876471118` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cl_llt` | `0.25233954270720266` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cdi_llt` | `0.0022009661990599855` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/e_llt` | `0.9208891957552111` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/relative_cl_gap` | `0.03787765530046283` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/relative_cdi_gap` | `0.11840623743203416` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/relative_e_difference` | `0.050006755349259135` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/same_cl_target` | `0.25233954270720266` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/vlm_alpha_at_same_cl_rad` | `0.05196844276141886` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/vlm_cdi_at_same_cl` | `0.0020961447989235912` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/relative_cdi_gap_at_same_cl` | `0.0476251748805423` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cl_effect_absolute` | `0.00016357711491359886` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/efficiency_effect_absolute` | `9.703101143217945e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/cl_effect_fraction_of_llt_drag` | `0.074320593829865` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/efficiency_effect_fraction_of_llt_drag` | `0.04408564360216912` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/decomposition_roundoff` | `-2.7755575615628914e-17` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/residual` | `1.824929096727601e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/0/max_panel_width_m` | `0.12270614261456132` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/ny` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cl_vlm` | `0.272009199786332` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cdi_vlm` | `0.0012972385817023068` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/e_vlm` | `0.9077525144131966` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cl_llt` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cdi_llt` | `0.0014208764734762762` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/e_llt` | `0.8607819175717625` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/relative_cl_gap` | `0.018774266694212782` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/relative_cdi_gap` | `0.08701522903780678` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/relative_e_difference` | `0.05456736007412499` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/same_cl_target` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/vlm_alpha_at_same_cl_rad` | `0.05095667419111409` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/vlm_cdi_at_same_cl` | `0.0013473548748715331` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/relative_cdi_gap_at_same_cl` | `0.051743835567118035` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cl_effect_absolute` | `5.285100698417238e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/efficiency_effect_absolute` | `7.078688478979697e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/cl_effect_fraction_of_llt_drag` | `0.037196060298520184` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/efficiency_effect_fraction_of_llt_drag` | `0.049819168739286476` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/decomposition_roundoff` | `-1.249000902703301e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/residual` | `2.275957200481571e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/1/max_panel_width_m` | `0.12271538285719945` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/ny` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cl_vlm` | `0.2900602280623624` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cdi_vlm` | `0.0008023373041232369` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/e_vlm` | `0.8344676775566786` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cl_llt` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cdi_llt` | `0.0008624499494412176` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/e_llt` | `0.7909357573434621` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/relative_cl_gap` | `0.009291939254773562` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/relative_cdi_gap` | `0.06969986531615868` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/relative_e_difference` | `0.05503850320211634` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/same_cl_target` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/vlm_alpha_at_same_cl_rad` | `0.05046895445908576` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/vlm_cdi_at_same_cl` | `0.0008174582698390826` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/relative_cdi_gap_at_same_cl` | `0.052167293454287056` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cl_effect_absolute` | `1.595320103581655e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/efficiency_effect_absolute` | `4.415944428216422e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/cl_effect_fraction_of_llt_drag` | `0.018497538374432687` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/efficiency_effect_fraction_of_llt_drag` | `0.05120232694172587` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/decomposition_roundoff` | `-1.249000902703301e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/residual` | `4.288236432614667e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/2/max_panel_width_m` | `0.1227176929830903` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/ar` | `80` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/ny` | `1024` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cl_vlm` | `0.3006598409603533` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cdi_vlm` | `0.00047447451906217525` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/e_vlm` | `0.7580504202570049` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cl_llt` | `0.30205472754839835` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cdi_llt` | `0.0005041101180511026` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/e_llt` | `0.7201218703527816` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/relative_cl_gap` | `0.004617992902698487` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/relative_cdi_gap` | `0.05878794717213587` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/relative_e_difference` | `0.052669626442038764` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/same_cl_target` | `0.30205472754839835` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/vlm_alpha_at_same_cl_rad` | `0.05023197088503566` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/vlm_cdi_at_same_cl` | `0.0004788873027095547` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/relative_cdi_gap_at_same_cl` | `0.05003433662283896` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cl_effect_absolute` | `4.645203313656662e-06` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/efficiency_effect_absolute` | `2.4990395675270722e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/cl_effect_fraction_of_llt_drag` | `0.00921465994694759` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/efficiency_effect_fraction_of_llt_drag` | `0.04957328722518806` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/decomposition_roundoff` | `-2.1510571102112408e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/residual` | `4.794775687599895e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/3/max_panel_width_m` | `0.12271827051864552` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/ar` | `160` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/ny` | `2048` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cl_vlm` | `0.30670283035006835` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cdi_vlm` | `0.00027278588212466254` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/e_vlm` | `0.6860311288063354` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cl_llt` | `0.30741088680954853` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cdi_llt` | `0.00028749385802233904` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/e_llt` | `0.6539432498726192` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/relative_cl_gap` | `0.002303290123615076` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/relative_cdi_gap` | `0.05115926997137332` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/relative_e_difference` | `0.04906829291374537` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/same_cl_target` | `0.30741088680954853` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/vlm_alpha_at_same_cl_rad` | `0.05011543037582536` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/vlm_cdi_at_same_cl` | `0.00027404684705877085` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/relative_cdi_gap_at_same_cl` | `0.046773211282041816` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cl_effect_absolute` | `1.322838330849183e-06` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/efficiency_effect_absolute` | `1.3385137566827323e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/cl_effect_fraction_of_llt_drag` | `0.0046012751018366105` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/efficiency_effect_fraction_of_llt_drag` | `0.046557994869536594` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/decomposition_roundoff` | `-1.1796119636642288e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/residual` | `9.631184738623233e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/baseline/4/max_panel_width_m` | `0.12271841490278129` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/ny` | `310` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cl_vlm` | `0.27196707256988606` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cdi_vlm` | `0.0012969238930191378` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/e_vlm` | `0.9076915522350301` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cl_llt` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cdi_llt` | `0.0014208764734762762` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/e_llt` | `0.8607819175717625` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/relative_cl_gap` | `0.018926233277996585` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/relative_cdi_gap` | `0.08723670408440187` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/relative_e_difference` | `0.054496538212138734` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/same_cl_target` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/vlm_alpha_at_same_cl_rad` | `0.050964567289431946` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/vlm_cdi_at_same_cl` | `0.001347445365619997` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/relative_cdi_gap_at_same_cl` | `0.05168014899748796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cl_effect_absolute` | `5.327471796298536e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/efficiency_effect_absolute` | `7.067786249415303e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/cl_effect_fraction_of_llt_drag` | `0.03749426424989999` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/efficiency_effect_fraction_of_llt_drag` | `0.04974243983450205` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/decomposition_roundoff` | `1.6653345369377348e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/residual` | `7.230327447871332e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/max_panel_width_m` | `0.12271538285719945` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/baseline_ny` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/tip_subdivisions` | `2` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/0/tip_semispan_fraction` | `0.05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/ny` | `414` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cl_vlm` | `0.27194669244904257` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cdi_vlm` | `0.001296768636841894` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/e_vlm` | `0.9076641772795779` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cl_llt` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cdi_llt` | `0.0014208764734762762` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/e_llt` | `0.8607819175717625` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/relative_cl_gap` | `0.018999751008408672` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/relative_cdi_gap` | `0.08734597197653893` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/relative_e_difference` | `0.05446473578356392` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/same_cl_target` | `0.27721368340995556` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/vlm_alpha_at_same_cl_rad` | `0.050968386655759736` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/vlm_cdi_at_same_cl` | `0.001347486004281058` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/relative_cdi_gap_at_same_cl` | `0.0516515478757017` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cl_effect_absolute` | `5.347967545637632e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/efficiency_effect_absolute` | `7.062816117800593e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/cl_effect_fraction_of_llt_drag` | `0.03763851147843578` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/efficiency_effect_fraction_of_llt_drag` | `0.04970746049810302` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/decomposition_roundoff` | `-1.3877787807814457e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/residual` | `8.007483565108942e-15` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/max_panel_width_m` | `0.12271538285719945` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/baseline_ny` | `256` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/tip_subdivisions` | `4` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/1/tip_semispan_fraction` | `0.05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/ny` | `618` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cl_vlm` | `0.290030979530907` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cdi_vlm` | `0.0008022272820738464` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/e_vlm` | `0.8344138177738735` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cl_llt` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cdi_llt` | `0.0008624499494412176` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/e_llt` | `0.7909357573434621` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/relative_cl_gap` | `0.009391838355285897` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/relative_cdi_gap` | `0.06982743451534734` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/relative_e_difference` | `0.05497040692210242` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/same_cl_target` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/vlm_alpha_at_same_cl_rad` | `0.050474044062977064` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/vlm_cdi_at_same_cl` | `0.0008175110351743732` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/relative_cdi_gap_at_same_cl` | `0.05210611270365362` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cl_effect_absolute` | `1.6123907227759654e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/efficiency_effect_absolute` | `4.409876013961164e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/cl_effect_fraction_of_llt_drag` | `0.018695470082879972` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/efficiency_effect_fraction_of_llt_drag` | `0.051131964432467286` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/decomposition_roundoff` | `-8.326672684688674e-17` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/residual` | `1.0103029524088925e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/max_panel_width_m` | `0.1227176929830903` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/baseline_ny` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/tip_subdivisions` | `2` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/2/tip_semispan_fraction` | `0.05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/ny` | `826` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/nx` | `8` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cl_vlm` | `0.29001664100970476` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cdi_vlm` | `0.0008021708208658902` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/e_vlm` | `0.8343900413284026` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cl_llt` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cdi_llt` | `0.0008624499494412176` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/e_llt` | `0.7909357573434621` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/relative_cl_gap` | `0.00944081193787294` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/relative_cdi_gap` | `0.06989290058441344` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/relative_e_difference` | `0.05494034576321538` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/same_cl_target` | `0.29278072880942796` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/vlm_alpha_at_same_cl_rad` | `0.05047653951685323` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/vlm_cdi_at_same_cl` | `0.0008175343306424239` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/relative_cdi_gap_at_same_cl` | `0.052079101897906765` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cl_effect_absolute` | `1.620758631579308e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/efficiency_effect_absolute` | `4.4071542259534386e-05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/cl_effect_fraction_of_llt_drag` | `0.018792494945699567` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/efficiency_effect_fraction_of_llt_drag` | `0.051100405638713674` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/decomposition_roundoff` | `-1.942890293094024e-16` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/residual` | `1.3655743202889425e-14` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/max_panel_width_m` | `0.1227176929830903` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/baseline_ny` | `512` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/tip_subdivisions` | `4` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/tip_refinement/3/tip_semispan_fraction` | `0.05` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/0/ar` | `20` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/0/max_coordinate_difference` | `0.0` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/0/area_difference` | `0.0` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/1/ar` | `40` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/1/max_coordinate_difference` | `0.0` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |
| `/mesh_reconstruction/1/area_difference` | `0.0` | Computed observation/algebraic result from SYNTHETIC inputs (no assertion) |

## Driver literal inventory

Sources: MIT Lecture9 for the coefficient/efficiency law; JVL for VLM. Integers in path/index/format operations are structural settings. Fractions .25/.75 and factors2 are sourced geometry definitions. The outer .05 semispan fraction and 2/4 subdivisions are declared numerical experiments. No number is a validity threshold.

| Appendix driver line | Literal | Provenance |
|---|---|---|
| 12 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `10` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `128` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `20` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `256` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `40` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `512` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `80` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `1024` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `160` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 20 | `2048` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 21 | `8` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 22 | `0.05` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `128` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `256` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `512` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `1024` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `2048` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 23 | `4096` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 24 | `20` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 24 | `40` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 25 | `0.05` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 26 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 26 | `4` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 38 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 39 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 40 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 41 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 42 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 47 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 48 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 49 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 53 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 58 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 68 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 69 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 70 | `0` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 70 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 71 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 71 | `0.0` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 74 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 74 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 76 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 76 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 76 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 84 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 85 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 85 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 86 | `0.25` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 87 | `0.75` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 89 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 101 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 102 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 103 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 120 | `6` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 123 | `7` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 123 | `7` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 124 | `7` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 124 | `7` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 128 | `1` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |
| 147 | `2` | SYNTHETIC input/count, structural setting or sourced algebraic factor; categories above. |

## Owner-supplied independent record (informational)

These are the exact supplied rounded SYNTHETIC benchmark results, not fresh acceptance thresholds.

| Record path | Value | Provenance |
|---|---|---|
| `/supplied/fit_range_ar/0` | `20` | Owner-supplied independent test-setting/result record. |
| `/supplied/fit_range_ar/1` | `160` | Owner-supplied independent test-setting/result record. |
| `/supplied/cl_fit_p` | `1.00897` | Owner-supplied independent test-setting/result record. |
| `/supplied/cdi_fit_p` | `0.25445` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/0/ar` | `10` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/0/ny` | `128` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/0/nx` | `8` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/0/cl_gap_percent` | `3.787766` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/0/cdi_gap_percent` | `11.840624` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/1/ar` | `20` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/1/ny` | `256` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/1/nx` | `8` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/1/cl_gap_percent` | `1.877427` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/1/cdi_gap_percent` | `8.701523` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/2/ar` | `40` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/2/ny` | `512` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/2/nx` | `8` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/2/cl_gap_percent` | `0.929194` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/2/cdi_gap_percent` | `6.969987` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/3/ar` | `80` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/3/ny` | `1024` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/3/nx` | `8` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/3/cl_gap_percent` | `0.461799` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/3/cdi_gap_percent` | `5.878795` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/4/ar` | `160` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/4/ny` | `2048` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/4/nx` | `8` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/4/cl_gap_percent` | `0.230329` | Owner-supplied independent test-setting/result record. |
| `/supplied/rows/4/cdi_gap_percent` | `5.115927` | Owner-supplied independent test-setting/result record. |
