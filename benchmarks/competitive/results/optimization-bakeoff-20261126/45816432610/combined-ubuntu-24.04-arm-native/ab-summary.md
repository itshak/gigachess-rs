| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.059x | 0.941x | 1.431x |
| materialize | 1.059x | 0.932x | 1.303x |
| perft_bulk_d4 | 0.964x | 0.899x | 1.298x |
| perft_full_d3 | 1.047x | 1.030x | 1.102x |
| fen_parse | 0.990x | 0.987x | 1.240x |
| fen_write_native_ep | 0.989x | 0.975x | 1.597x |
| snapshot | 0.972x | 0.969x | 0.951x |
| san_parse_all | 0.993x | 0.985x | 1.811x |
| san_render_native | 0.983x | 0.971x | 2.519x |
| apply_children_full | 1.057x | 1.040x | 1.185x |
| make_unmake_full | 1.026x | 1.004x | 1.475x |
| startpos | 1.038x | 1.038x | 1.306x |
| chess960_materialize | 1.107x | 1.066x | 1.471x |
| chess960_perft_d3 | 1.093x | 0.983x | 1.440x |
| polyglot_key | 0.992x | 0.986x | 25.777x |
| polyglot_recompute | 0.996x | 0.993x | 1.070x |
| repetition_key | 0.950x | 0.935x | 18.369x |
| corpus_count | 0.861x | 0.831x | 1.256x |
| corpus_materialize | 1.025x | 1.018x | 1.215x |
| corpus_uci_sorted | 1.000x | 1.000x | 1.459x |
| replay_trusted_native | 1.048x | 1.026x | 0.549x |
| replay_checked_native | 1.042x | 1.037x | 1.021x |

Cold first legal move list: 0.013 ms -> 0.007 ms (1.73x).
