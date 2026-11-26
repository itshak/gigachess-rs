| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.970x | 0.926x | 1.335x |
| materialize | 0.979x | 0.961x | 1.500x |
| perft_bulk_d4 | 1.008x | 0.975x | 1.019x |
| perft_full_d3 | 0.881x | 0.798x | 1.308x |
| fen_parse | 1.060x | 1.013x | 1.107x |
| fen_write_native_ep | 0.978x | 0.916x | 1.903x |
| snapshot | 0.610x | 0.606x | 0.935x |
| san_parse_all | 0.906x | 0.867x | 1.727x |
| san_render_native | 0.998x | 0.954x | 2.080x |
| apply_children_full | 0.987x | 0.951x | 1.588x |
| make_unmake_full | 1.068x | 1.035x | 1.248x |
| startpos | 1.024x | 1.024x | 1.294x |
| chess960_materialize | 0.930x | 0.870x | 1.386x |
| chess960_perft_d3 | 0.942x | 0.914x | 1.334x |
| polyglot_key | 0.964x | 0.934x | 35.793x |
| polyglot_recompute | 0.988x | 0.987x | 1.017x |
| repetition_key | 1.013x | 0.804x | 18.384x |
| corpus_count | 1.076x | 0.985x | 0.952x |
| corpus_materialize | 1.006x | 1.004x | 1.026x |
| corpus_uci_sorted | 1.008x | 1.005x | 1.563x |
| replay_trusted_native | 0.905x | 0.895x | 0.802x |
| replay_checked_native | 0.942x | 0.914x | 1.045x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.83x).
