| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.750x | 1.235x |
| materialize | 1.036x | 0.717x | 0.975x |
| perft_bulk_d4 | 1.029x | 0.707x | 1.287x |
| perft_full_d3 | 1.108x | 0.987x | 1.380x |
| fen_parse | 1.032x | 0.920x | 1.181x |
| fen_write_native_ep | 1.097x | 0.791x | 1.759x |
| snapshot | 1.032x | 0.648x | 0.882x |
| san_parse_all | 1.141x | 1.052x | 1.942x |
| san_render_native | 1.136x | 0.931x | 2.836x |
| apply_children_full | 0.972x | 0.817x | 0.901x |
| make_unmake_full | 1.111x | 0.826x | 1.283x |
| startpos | 1.524x | 1.524x | 1.478x |
| chess960_materialize | 0.995x | 0.927x | 1.402x |
| chess960_perft_d3 | 0.937x | 0.861x | 1.288x |
| polyglot_key | 0.984x | 0.884x | 42.053x |
| polyglot_recompute | 0.778x | 0.745x | 0.963x |
| repetition_key | 0.973x | 0.821x | 19.690x |
| corpus_count | 0.762x | 0.667x | 1.162x |
| corpus_materialize | 0.969x | 0.887x | 1.118x |
| corpus_uci_sorted | 1.074x | 1.005x | 1.605x |
| replay_trusted_native | 0.831x | 0.770x | 0.496x |
| replay_checked_native | 0.992x | 0.860x | 1.008x |

Cold first legal move list: 0.008 ms -> 0.012 ms (0.67x).
