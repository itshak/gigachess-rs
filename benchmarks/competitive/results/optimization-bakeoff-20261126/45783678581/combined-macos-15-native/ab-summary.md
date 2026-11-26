| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.051x | 0.767x | 1.244x |
| materialize | 1.115x | 0.729x | 1.002x |
| perft_bulk_d4 | 1.025x | 0.751x | 1.332x |
| perft_full_d3 | 1.219x | 0.972x | 1.417x |
| fen_parse | 1.005x | 0.870x | 1.255x |
| fen_write_native_ep | 1.109x | 0.834x | 1.271x |
| snapshot | 1.033x | 0.915x | 0.982x |
| san_parse_all | 1.030x | 0.928x | 1.822x |
| san_render_native | 1.099x | 1.008x | 2.615x |
| apply_children_full | 1.384x | 1.248x | 1.210x |
| make_unmake_full | 1.178x | 0.999x | 1.304x |
| startpos | 1.026x | 1.026x | 1.207x |
| chess960_materialize | 1.082x | 1.036x | 1.452x |
| chess960_perft_d3 | 1.013x | 0.969x | 1.368x |
| polyglot_key | 0.882x | 0.821x | 39.049x |
| polyglot_recompute | 0.926x | 0.853x | 1.053x |
| repetition_key | 0.997x | 0.921x | 19.297x |
| corpus_count | 0.982x | 0.836x | 1.358x |
| corpus_materialize | 0.897x | 0.802x | 1.107x |
| corpus_uci_sorted | 1.050x | 1.029x | 1.388x |
| replay_trusted_native | 1.214x | 0.997x | 0.674x |
| replay_checked_native | 1.125x | 1.123x | 1.213x |

Cold first legal move list: 0.010 ms -> 0.007 ms (1.51x).
