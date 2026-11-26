| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.983x | 1.293x |
| materialize | 1.000x | 0.988x | 1.185x |
| perft_bulk_d4 | 0.999x | 0.964x | 1.217x |
| perft_full_d3 | 0.955x | 0.907x | 1.101x |
| fen_parse | 1.021x | 0.979x | 1.208x |
| fen_write_native_ep | 1.013x | 0.989x | 1.721x |
| snapshot | 1.285x | 1.282x | 1.249x |
| san_parse_all | 1.023x | 0.983x | 2.049x |
| san_render_native | 0.965x | 0.899x | 2.137x |
| apply_children_full | 0.892x | 0.863x | 1.070x |
| make_unmake_full | 0.903x | 0.869x | 1.100x |
| startpos | 1.000x | 1.000x | 1.249x |
| chess960_materialize | 0.942x | 0.881x | 1.215x |
| chess960_perft_d3 | 0.929x | 0.880x | 1.311x |
| polyglot_key | 1.003x | 1.002x | 25.648x |
| polyglot_recompute | 1.004x | 1.001x | 1.033x |
| repetition_key | 1.009x | 1.001x | 18.506x |
| corpus_count | 1.014x | 1.013x | 1.243x |
| corpus_materialize | 1.004x | 0.998x | 1.123x |
| corpus_uci_sorted | 1.014x | 1.008x | 1.793x |
| replay_trusted_native | 0.924x | 0.874x | 0.709x |
| replay_checked_native | 0.960x | 0.957x | 0.963x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.01x).
