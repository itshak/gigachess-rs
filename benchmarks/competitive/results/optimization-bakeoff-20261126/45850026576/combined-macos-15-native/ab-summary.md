| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.851x | 0.650x | 1.330x |
| materialize | 0.968x | 0.799x | 1.177x |
| perft_bulk_d4 | 1.033x | 0.871x | 1.190x |
| perft_full_d3 | 1.036x | 0.884x | 1.295x |
| fen_parse | 1.045x | 0.791x | 1.240x |
| fen_write_native_ep | 1.012x | 0.857x | 1.657x |
| snapshot | 1.019x | 0.811x | 1.026x |
| san_parse_all | 0.973x | 0.836x | 1.829x |
| san_render_native | 1.110x | 0.997x | 2.525x |
| apply_children_full | 1.025x | 0.740x | 1.062x |
| make_unmake_full | 0.947x | 0.779x | 1.322x |
| startpos | 0.785x | 0.785x | 0.933x |
| chess960_materialize | 1.007x | 0.987x | 1.577x |
| chess960_perft_d3 | 0.853x | 0.802x | 1.420x |
| polyglot_key | 1.065x | 1.016x | 38.581x |
| polyglot_recompute | 0.844x | 0.829x | 0.829x |
| repetition_key | 1.055x | 0.989x | 20.643x |
| corpus_count | 1.090x | 0.964x | 1.578x |
| corpus_materialize | 0.848x | 0.797x | 0.981x |
| corpus_uci_sorted | 1.173x | 0.959x | 1.332x |
| replay_trusted_native | 1.097x | 0.999x | 0.713x |
| replay_checked_native | 1.087x | 1.064x | 1.160x |

Cold first legal move list: 0.006 ms -> 0.004 ms (1.40x).
