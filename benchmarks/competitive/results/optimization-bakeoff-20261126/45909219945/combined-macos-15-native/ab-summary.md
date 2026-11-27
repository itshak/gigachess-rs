| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.168x | 0.952x | 1.408x |
| materialize | 1.234x | 1.075x | 1.061x |
| perft_bulk_d4 | 1.176x | 0.791x | 1.258x |
| perft_full_d3 | 1.121x | 0.852x | 1.296x |
| fen_parse | 1.076x | 0.930x | 1.186x |
| fen_write_native_ep | 1.163x | 0.916x | 1.862x |
| snapshot | 1.310x | 0.884x | 1.057x |
| san_parse_all | 1.040x | 0.958x | 1.774x |
| san_render_native | 1.122x | 0.938x | 2.783x |
| apply_children_full | 1.219x | 1.036x | 1.102x |
| make_unmake_full | 1.072x | 0.897x | 1.219x |
| startpos | 1.006x | 1.006x | 1.147x |
| chess960_materialize | 1.109x | 1.065x | 1.506x |
| chess960_perft_d3 | 1.211x | 1.157x | 1.495x |
| polyglot_key | 1.187x | 1.161x | 41.605x |
| polyglot_recompute | 1.075x | 0.990x | 0.981x |
| repetition_key | 1.069x | 1.055x | 20.259x |
| corpus_count | 1.291x | 1.221x | 1.318x |
| corpus_materialize | 1.103x | 1.015x | 1.164x |
| corpus_uci_sorted | 1.127x | 0.975x | 1.402x |
| replay_trusted_native | 1.040x | 1.036x | 0.672x |
| replay_checked_native | 1.159x | 1.138x | 1.277x |

Cold first legal move list: 0.003 ms -> 0.004 ms (0.69x).
