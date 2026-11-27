| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.028x | 0.850x | 1.716x |
| materialize | 0.899x | 0.720x | 1.051x |
| perft_bulk_d4 | 1.025x | 0.890x | 1.192x |
| perft_full_d3 | 0.988x | 0.700x | 1.272x |
| fen_parse | 0.987x | 0.868x | 1.104x |
| fen_write_native_ep | 1.163x | 1.053x | 1.930x |
| snapshot | 0.926x | 0.667x | 0.968x |
| san_parse_all | 1.017x | 0.815x | 1.604x |
| san_render_native | 1.047x | 0.837x | 2.651x |
| apply_children_full | 1.080x | 0.889x | 1.053x |
| make_unmake_full | 1.093x | 0.935x | 1.303x |
| startpos | 0.994x | 0.994x | 1.228x |
| chess960_materialize | 1.026x | 1.021x | 1.539x |
| chess960_perft_d3 | 1.017x | 0.984x | 1.510x |
| polyglot_key | 0.904x | 0.874x | 32.662x |
| polyglot_recompute | 0.856x | 0.761x | 0.850x |
| repetition_key | 0.921x | 0.883x | 18.906x |
| corpus_count | 1.010x | 0.941x | 1.202x |
| corpus_materialize | 0.887x | 0.861x | 1.152x |
| corpus_uci_sorted | 0.897x | 0.707x | 1.177x |
| replay_trusted_native | 1.090x | 1.072x | 0.644x |
| replay_checked_native | 1.172x | 1.018x | 1.420x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.76x).
