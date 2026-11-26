| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.743x | 1.391x |
| materialize | 0.987x | 0.825x | 1.108x |
| perft_bulk_d4 | 0.986x | 0.840x | 1.235x |
| perft_full_d3 | 1.157x | 0.897x | 1.444x |
| fen_parse | 1.036x | 0.819x | 1.189x |
| fen_write_native_ep | 0.937x | 0.819x | 1.514x |
| snapshot | 0.637x | 0.516x | 0.645x |
| san_parse_all | 1.156x | 0.930x | 1.883x |
| san_render_native | 1.003x | 0.777x | 3.032x |
| apply_children_full | 1.124x | 1.026x | 1.150x |
| make_unmake_full | 1.034x | 0.937x | 1.386x |
| startpos | 0.502x | 0.502x | 1.243x |
| chess960_materialize | 0.873x | 0.853x | 1.558x |
| chess960_perft_d3 | 1.036x | 1.021x | 1.403x |
| polyglot_key | 0.938x | 0.810x | 34.546x |
| polyglot_recompute | 0.871x | 0.817x | 0.989x |
| repetition_key | 0.985x | 0.921x | 20.550x |
| corpus_count | 0.860x | 0.844x | 1.149x |
| corpus_materialize | 1.165x | 0.978x | 1.319x |
| corpus_uci_sorted | 0.932x | 0.895x | 1.409x |
| replay_trusted_native | 1.083x | 1.069x | 0.653x |
| replay_checked_native | 0.909x | 0.724x | 1.153x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.84x).
