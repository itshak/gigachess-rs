| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.977x | 0.906x | 1.389x |
| materialize | 0.967x | 0.889x | 1.232x |
| perft_bulk_d4 | 0.944x | 0.861x | 1.291x |
| perft_full_d3 | 1.019x | 0.977x | 1.211x |
| fen_parse | 1.140x | 1.099x | 1.249x |
| fen_write_native_ep | 1.045x | 1.011x | 1.675x |
| snapshot | 0.954x | 0.949x | 0.900x |
| san_parse_all | 0.979x | 0.963x | 2.013x |
| san_render_native | 0.986x | 0.966x | 2.210x |
| apply_children_full | 1.003x | 0.949x | 1.229x |
| make_unmake_full | 0.941x | 0.921x | 1.197x |
| startpos | 1.080x | 1.080x | 1.252x |
| chess960_materialize | 1.077x | 1.009x | 1.334x |
| chess960_perft_d3 | 1.064x | 0.994x | 1.429x |
| polyglot_key | 0.989x | 0.982x | 27.161x |
| polyglot_recompute | 1.002x | 0.994x | 1.036x |
| repetition_key | 0.877x | 0.862x | 18.568x |
| corpus_count | 0.887x | 0.779x | 1.319x |
| corpus_materialize | 0.948x | 0.921x | 1.127x |
| corpus_uci_sorted | 0.997x | 0.990x | 1.691x |
| replay_trusted_native | 1.041x | 1.040x | 0.793x |
| replay_checked_native | 1.043x | 1.036x | 1.058x |

Cold first legal move list: 0.013 ms -> 0.008 ms (1.73x).
