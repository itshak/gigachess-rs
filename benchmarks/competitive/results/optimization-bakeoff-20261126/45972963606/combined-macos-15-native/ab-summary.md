| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.020x | 0.892x | 1.350x |
| materialize | 1.147x | 0.801x | 1.130x |
| perft_bulk_d4 | 0.920x | 0.804x | 1.306x |
| perft_full_d3 | 1.152x | 0.951x | 1.359x |
| fen_parse | 0.995x | 0.821x | 1.200x |
| fen_write_native_ep | 0.995x | 0.678x | 1.856x |
| snapshot | 1.114x | 0.890x | 0.879x |
| san_parse_all | 0.883x | 0.745x | 1.722x |
| san_render_native | 1.014x | 0.676x | 2.671x |
| apply_children_full | 1.022x | 0.958x | 1.015x |
| make_unmake_full | 1.002x | 0.761x | 1.281x |
| startpos | 0.881x | 0.881x | 1.141x |
| chess960_materialize | 0.964x | 0.919x | 1.395x |
| chess960_perft_d3 | 1.088x | 0.901x | 1.501x |
| polyglot_key | 0.954x | 0.906x | 37.763x |
| polyglot_recompute | 0.956x | 0.899x | 0.889x |
| repetition_key | 1.099x | 1.076x | 21.385x |
| corpus_count | 1.028x | 0.935x | 1.070x |
| corpus_materialize | 1.044x | 0.991x | 1.068x |
| corpus_uci_sorted | 1.062x | 0.959x | 1.274x |
| replay_trusted_native | 1.029x | 0.939x | 0.653x |
| replay_checked_native | 0.725x | 0.706x | 1.070x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.55x).
