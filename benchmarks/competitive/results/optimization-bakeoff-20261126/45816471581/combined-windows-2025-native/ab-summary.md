| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.989x | 1.325x |
| materialize | 1.015x | 0.991x | 1.181x |
| perft_bulk_d4 | 1.081x | 0.982x | 1.214x |
| perft_full_d3 | 1.015x | 0.975x | 1.278x |
| fen_parse | 0.890x | 0.866x | 1.138x |
| fen_write_native_ep | 1.012x | 0.984x | 1.570x |
| snapshot | 0.990x | 0.974x | 0.995x |
| san_parse_all | 1.047x | 0.984x | 2.058x |
| san_render_native | 1.042x | 0.954x | 3.749x |
| apply_children_full | 1.028x | 0.985x | 1.288x |
| make_unmake_full | 1.027x | 0.991x | 1.292x |
| startpos | 1.001x | 1.001x | 1.246x |
| chess960_materialize | 1.009x | 0.964x | 1.263x |
| chess960_perft_d3 | 0.838x | 0.834x | 1.239x |
| polyglot_key | 0.941x | 0.938x | 27.542x |
| polyglot_recompute | 0.966x | 0.954x | 0.998x |
| repetition_key | 0.876x | 0.785x | 18.662x |
| corpus_count | 0.913x | 0.880x | 1.173x |
| corpus_materialize | 0.987x | 0.980x | 1.119x |
| corpus_uci_sorted | 1.049x | 1.003x | 1.450x |
| replay_trusted_native | 0.974x | 0.934x | 0.714x |
| replay_checked_native | 1.009x | 1.007x | 1.044x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.03x).
