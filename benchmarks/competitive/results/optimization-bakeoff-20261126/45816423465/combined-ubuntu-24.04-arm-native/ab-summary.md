| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.940x | 1.387x |
| materialize | 1.005x | 0.967x | 1.242x |
| perft_bulk_d4 | 0.919x | 0.895x | 1.239x |
| perft_full_d3 | 1.014x | 0.983x | 1.071x |
| fen_parse | 0.998x | 0.991x | 1.238x |
| fen_write_native_ep | 1.000x | 0.998x | 1.575x |
| snapshot | 0.916x | 0.912x | 0.948x |
| san_parse_all | 0.996x | 0.988x | 1.820x |
| san_render_native | 0.988x | 0.981x | 2.513x |
| apply_children_full | 1.054x | 1.034x | 1.183x |
| make_unmake_full | 1.030x | 1.007x | 1.479x |
| startpos | 1.072x | 1.072x | 1.301x |
| chess960_materialize | 1.044x | 0.993x | 1.382x |
| chess960_perft_d3 | 1.024x | 0.973x | 1.346x |
| polyglot_key | 1.020x | 1.009x | 25.774x |
| polyglot_recompute | 1.001x | 1.001x | 1.074x |
| repetition_key | 0.978x | 0.966x | 18.414x |
| corpus_count | 0.814x | 0.792x | 1.201x |
| corpus_materialize | 0.961x | 0.950x | 1.154x |
| corpus_uci_sorted | 0.997x | 0.997x | 1.451x |
| replay_trusted_native | 1.045x | 1.024x | 0.555x |
| replay_checked_native | 1.040x | 1.035x | 1.024x |

Cold first legal move list: 0.011 ms -> 0.007 ms (1.58x).
