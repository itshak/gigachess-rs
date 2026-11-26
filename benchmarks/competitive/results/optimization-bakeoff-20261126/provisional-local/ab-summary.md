| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.178x | 0.918x | 1.163x |
| materialize | 1.226x | 0.829x | 1.139x |
| perft_bulk_d4 | 1.568x | 1.123x | 1.050x |
| perft_full_d3 | 1.800x | 1.322x | 0.858x |
| fen_parse | 1.231x | 0.957x | 1.022x |
| fen_write_native_ep | 1.122x | 0.976x | 1.172x |
| snapshot | 1.034x | 0.770x | 0.622x |
| san_parse_all | 1.811x | 1.472x | 1.666x |
| san_render_native | 1.921x | 1.285x | 1.917x |
| apply_children_full | 1.230x | 0.935x | 0.777x |
| make_unmake_full | 1.180x | 0.806x | 0.903x |
| startpos | 1.006x | 1.006x | 0.950x |
| chess960_materialize | 0.754x | 0.706x | 1.122x |
| chess960_perft_d3 | 0.842x | 0.784x | 1.056x |
| polyglot_key | 0.956x | 0.924x | 28.497x |
| polyglot_recompute | 0.983x | 0.967x | 0.968x |
| repetition_key | 1.035x | 0.993x | 14.448x |
| corpus_count | 0.912x | 0.823x | 1.329x |
| corpus_materialize | 1.024x | 0.842x | 1.019x |
| corpus_uci_sorted | 1.175x | 1.096x | 0.812x |
| replay_trusted_native | 0.932x | 0.823x | 0.425x |
| replay_checked_native | 0.703x | 0.488x | 0.747x |

Cold first legal move list: 236.704 ms → 0.023 ms (10474.10x).
