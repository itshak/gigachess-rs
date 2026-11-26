| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.904x | 1.350x |
| materialize | 1.034x | 0.931x | 1.025x |
| perft_bulk_d4 | 0.931x | 0.752x | 1.121x |
| perft_full_d3 | 1.052x | 0.917x | 1.305x |
| fen_parse | 1.103x | 0.857x | 1.270x |
| fen_write_native_ep | 1.046x | 0.880x | 1.679x |
| snapshot | 1.033x | 0.952x | 0.984x |
| san_parse_all | 1.014x | 0.933x | 1.882x |
| san_render_native | 1.032x | 0.829x | 2.580x |
| apply_children_full | 1.105x | 1.028x | 0.937x |
| make_unmake_full | 1.095x | 0.941x | 1.179x |
| startpos | 1.158x | 1.158x | 1.255x |
| chess960_materialize | 0.921x | 0.771x | 1.266x |
| chess960_perft_d3 | 0.773x | 0.613x | 1.223x |
| polyglot_key | 1.083x | 1.028x | 40.447x |
| polyglot_recompute | 0.998x | 0.984x | 1.045x |
| repetition_key | 0.978x | 0.977x | 19.833x |
| corpus_count | 0.725x | 0.601x | 0.874x |
| corpus_materialize | 0.900x | 0.876x | 1.108x |
| corpus_uci_sorted | 0.995x | 0.937x | 1.390x |
| replay_trusted_native | 1.141x | 0.952x | 0.555x |
| replay_checked_native | 1.054x | 0.935x | 0.968x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.09x).
