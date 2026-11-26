| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.912x | 0.754x | 1.415x |
| materialize | 0.895x | 0.800x | 1.143x |
| perft_bulk_d4 | 0.981x | 0.818x | 1.106x |
| perft_full_d3 | 0.908x | 0.838x | 1.330x |
| fen_parse | 0.906x | 0.754x | 1.177x |
| fen_write_native_ep | 1.005x | 0.739x | 1.890x |
| snapshot | 0.992x | 0.628x | 0.972x |
| san_parse_all | 0.973x | 0.785x | 1.794x |
| san_render_native | 1.060x | 0.953x | 2.792x |
| apply_children_full | 1.028x | 0.929x | 1.142x |
| make_unmake_full | 0.968x | 0.816x | 1.363x |
| startpos | 1.224x | 1.224x | 1.194x |
| chess960_materialize | 0.872x | 0.721x | 1.254x |
| chess960_perft_d3 | 0.996x | 0.995x | 1.238x |
| polyglot_key | 1.033x | 1.002x | 36.265x |
| polyglot_recompute | 0.779x | 0.701x | 0.851x |
| repetition_key | 0.892x | 0.781x | 23.003x |
| corpus_count | 0.876x | 0.874x | 0.956x |
| corpus_materialize | 1.001x | 0.906x | 1.230x |
| corpus_uci_sorted | 0.904x | 0.744x | 1.282x |
| replay_trusted_native | 1.029x | 0.867x | 0.771x |
| replay_checked_native | 0.870x | 0.817x | 1.184x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.11x).
