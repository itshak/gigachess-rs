| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.006x | 0.999x | 1.494x |
| materialize | 0.943x | 0.903x | 1.519x |
| perft_bulk_d4 | 0.956x | 0.941x | 1.129x |
| perft_full_d3 | 0.999x | 0.982x | 1.278x |
| fen_parse | 1.001x | 0.992x | 1.114x |
| fen_write_native_ep | 0.971x | 0.944x | 1.993x |
| snapshot | 1.003x | 0.995x | 1.329x |
| san_parse_all | 0.996x | 0.981x | 2.046x |
| san_render_native | 1.193x | 1.141x | 2.083x |
| apply_children_full | 0.900x | 0.797x | 1.097x |
| make_unmake_full | 0.913x | 0.866x | 1.070x |
| startpos | 1.260x | 1.260x | 1.249x |
| chess960_materialize | 0.943x | 0.941x | 1.456x |
| chess960_perft_d3 | 0.970x | 0.955x | 1.452x |
| polyglot_key | 0.930x | 0.861x | 37.248x |
| polyglot_recompute | 1.021x | 1.013x | 0.986x |
| repetition_key | 0.914x | 0.849x | 22.297x |
| corpus_count | 1.013x | 1.005x | 1.085x |
| corpus_materialize | 0.987x | 0.986x | 1.043x |
| corpus_uci_sorted | 1.010x | 1.005x | 1.555x |
| replay_trusted_native | 1.145x | 1.015x | 0.986x |
| replay_checked_native | 1.009x | 0.995x | 1.062x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).
