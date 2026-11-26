| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.927x | 0.890x | 1.200x |
| materialize | 1.044x | 0.990x | 1.648x |
| perft_bulk_d4 | 0.881x | 0.809x | 1.046x |
| perft_full_d3 | 0.958x | 0.936x | 1.268x |
| fen_parse | 1.033x | 1.013x | 1.131x |
| fen_write_native_ep | 0.929x | 0.874x | 1.828x |
| snapshot | 1.003x | 1.001x | 1.121x |
| san_parse_all | 0.966x | 0.953x | 1.814x |
| san_render_native | 0.958x | 0.895x | 2.052x |
| apply_children_full | 0.970x | 0.948x | 1.344x |
| make_unmake_full | 0.939x | 0.911x | 1.152x |
| startpos | 1.403x | 1.403x | 1.807x |
| chess960_materialize | 0.890x | 0.859x | 1.397x |
| chess960_perft_d3 | 0.855x | 0.817x | 1.289x |
| polyglot_key | 0.997x | 0.997x | 40.640x |
| polyglot_recompute | 1.061x | 1.058x | 1.019x |
| repetition_key | 0.962x | 0.960x | 19.767x |
| corpus_count | 0.979x | 0.964x | 1.112x |
| corpus_materialize | 0.974x | 0.961x | 1.123x |
| corpus_uci_sorted | 1.007x | 1.005x | 1.567x |
| replay_trusted_native | 0.973x | 0.937x | 0.866x |
| replay_checked_native | 0.939x | 0.935x | 1.101x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.80x).
