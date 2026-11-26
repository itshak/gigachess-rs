| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.923x | 0.568x | 1.093x |
| materialize | 0.898x | 0.703x | 1.277x |
| perft_bulk_d4 | 0.999x | 0.947x | 0.877x |
| perft_full_d3 | 1.004x | 0.985x | 0.608x |
| fen_parse | 0.927x | 0.818x | 1.017x |
| fen_write_native_ep | 1.028x | 1.002x | 1.365x |
| snapshot | 1.323x | 1.319x | 0.790x |
| san_parse_all | 0.997x | 0.993x | 0.902x |
| san_render_native | 1.000x | 0.991x | 1.059x |
| apply_children_full | 1.035x | 1.032x | 0.646x |
| make_unmake_full | 0.997x | 0.983x | 0.751x |
| startpos | 1.122x | 1.122x | 0.703x |
| chess960_materialize | 1.080x | 1.055x | 1.182x |
| chess960_perft_d3 | 0.999x | 0.997x | 0.961x |
| polyglot_key | 1.051x | 0.998x | 33.508x |
| polyglot_recompute | 1.024x | 1.022x | 1.008x |
| repetition_key | 1.125x | 0.997x | 15.574x |
| corpus_count | 0.954x | 0.938x | 1.090x |
| corpus_materialize | 0.977x | 0.975x | 1.035x |
| corpus_uci_sorted | 0.961x | 0.956x | 0.605x |
| replay_trusted_native | 0.998x | 0.993x | 0.529x |
| replay_checked_native | 0.988x | 0.983x | 0.740x |

Cold first legal move list: 143.814 ms → 144.047 ms (1.00x).
