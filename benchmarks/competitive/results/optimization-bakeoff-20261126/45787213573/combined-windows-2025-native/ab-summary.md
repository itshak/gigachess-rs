| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.986x | 0.969x | 1.297x |
| materialize | 1.052x | 0.994x | 1.187x |
| perft_bulk_d4 | 0.984x | 0.917x | 1.299x |
| perft_full_d3 | 0.991x | 0.886x | 1.152x |
| fen_parse | 1.055x | 1.005x | 1.293x |
| fen_write_native_ep | 1.006x | 0.976x | 1.624x |
| snapshot | 1.030x | 0.985x | 1.040x |
| san_parse_all | 1.080x | 1.024x | 2.031x |
| san_render_native | 1.010x | 0.912x | 3.707x |
| apply_children_full | 0.878x | 0.840x | 0.993x |
| make_unmake_full | 0.904x | 0.869x | 1.136x |
| startpos | 1.003x | 1.003x | 1.105x |
| chess960_materialize | 1.011x | 0.996x | 1.224x |
| chess960_perft_d3 | 1.034x | 0.998x | 1.267x |
| polyglot_key | 0.927x | 0.869x | 25.445x |
| polyglot_recompute | 0.997x | 0.995x | 1.013x |
| repetition_key | 1.096x | 1.065x | 20.429x |
| corpus_count | 1.023x | 1.012x | 1.330x |
| corpus_materialize | 0.986x | 0.983x | 1.155x |
| corpus_uci_sorted | 1.014x | 1.002x | 1.416x |
| replay_trusted_native | 0.904x | 0.884x | 0.640x |
| replay_checked_native | 0.942x | 0.930x | 0.923x |

Cold first legal move list: 0.031 ms -> 0.032 ms (0.96x).
