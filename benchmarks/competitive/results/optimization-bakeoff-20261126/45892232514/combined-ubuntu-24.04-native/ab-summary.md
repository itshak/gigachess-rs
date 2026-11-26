| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.941x | 0.908x | 1.398x |
| materialize | 0.964x | 0.939x | 1.555x |
| perft_bulk_d4 | 0.875x | 0.855x | 1.031x |
| perft_full_d3 | 0.933x | 0.918x | 1.262x |
| fen_parse | 1.000x | 0.981x | 1.060x |
| fen_write_native_ep | 0.905x | 0.840x | 1.722x |
| snapshot | 0.671x | 0.661x | 0.861x |
| san_parse_all | 0.972x | 0.946x | 1.804x |
| san_render_native | 0.965x | 0.887x | 2.066x |
| apply_children_full | 0.931x | 0.906x | 1.307x |
| make_unmake_full | 0.895x | 0.788x | 1.084x |
| startpos | 0.992x | 0.992x | 1.314x |
| chess960_materialize | 0.922x | 0.886x | 1.483x |
| chess960_perft_d3 | 0.868x | 0.847x | 1.306x |
| polyglot_key | 0.962x | 0.924x | 37.748x |
| polyglot_recompute | 0.962x | 0.957x | 0.969x |
| repetition_key | 1.115x | 0.951x | 21.120x |
| corpus_count | 0.986x | 0.971x | 1.086x |
| corpus_materialize | 1.017x | 1.012x | 1.085x |
| corpus_uci_sorted | 1.026x | 1.023x | 1.620x |
| replay_trusted_native | 1.019x | 1.009x | 0.890x |
| replay_checked_native | 0.927x | 0.908x | 1.061x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.96x).
