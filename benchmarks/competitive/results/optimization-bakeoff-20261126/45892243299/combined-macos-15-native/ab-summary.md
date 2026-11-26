| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.958x | 0.910x | 1.535x |
| materialize | 0.962x | 0.900x | 1.107x |
| perft_bulk_d4 | 0.968x | 0.852x | 1.215x |
| perft_full_d3 | 1.021x | 0.952x | 1.349x |
| fen_parse | 0.973x | 0.880x | 1.173x |
| fen_write_native_ep | 1.090x | 0.947x | 1.913x |
| snapshot | 1.018x | 0.925x | 1.008x |
| san_parse_all | 1.012x | 0.915x | 1.733x |
| san_render_native | 0.990x | 0.841x | 2.729x |
| apply_children_full | 0.979x | 0.874x | 1.131x |
| make_unmake_full | 0.936x | 0.871x | 1.317x |
| startpos | 1.016x | 1.016x | 1.197x |
| chess960_materialize | 0.972x | 0.956x | 1.561x |
| chess960_perft_d3 | 0.941x | 0.940x | 1.491x |
| polyglot_key | 1.026x | 0.929x | 37.972x |
| polyglot_recompute | 1.187x | 0.972x | 1.167x |
| repetition_key | 0.926x | 0.877x | 18.010x |
| corpus_count | 1.076x | 0.991x | 1.184x |
| corpus_materialize | 1.098x | 0.994x | 1.152x |
| corpus_uci_sorted | 1.051x | 1.028x | 1.317x |
| replay_trusted_native | 1.030x | 1.022x | 0.662x |
| replay_checked_native | 1.052x | 1.005x | 1.218x |

Cold first legal move list: 0.003 ms -> 0.004 ms (0.87x).
