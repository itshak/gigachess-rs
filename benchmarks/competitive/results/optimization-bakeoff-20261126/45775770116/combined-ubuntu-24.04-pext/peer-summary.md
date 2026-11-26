| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.93x | 1.53x | 1.50x | 2.04x | 1.69x | 30/30 |
| materialize | 1.00x | 2.01x | 1.52x | 1.37x | 2.23x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.47x | 1.27x | 2.10x | 1.74x | 30/30 |
| perft_full_d3 | 0.97x | 0.90x | 0.97x | 0.87x | 1.25x | 1.12x | 15/30 |
| fen_parse | 1.14x | 1.46x | 1.62x | 1.19x | 1.95x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 3.46x | 7.12x | 1.35x | 20.09x | 1.66x | 30/30 |
| snapshot | 1.41x | 1.65x | 1.06x | 3.50x | 2.44x | 1.15x | 30/30 |
| san_parse_all | 1.01x | 1.83x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.04x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.15x | 1.59x | 1.00x | 3.37x | 0.98x | 1.15x | 24/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 0.81x | 1.08x | 86.39x | 146.60x | 213.89x | 158.26x | 5/5 |
| chess960_materialize | 0.99x | 1.52x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.22x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 30.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 20.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.91x | 2.25x | 1.35x | 2.68x | 2.56x | 10/10 |
| corpus_materialize | 0.99x | 1.51x | 1.46x | 1.18x | 1.85x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.43x | 1.87x | 1.42x | 1.77x | 0.97x | 8/10 |
| replay_trusted_native | 1.08x | 0.61x | 1.21x | 1.31x | 1.65x | 1.76x | 8/10 |
| replay_checked_native | 1.02x | 1.99x | 0.97x | 7.16x | 6.72x | 8.11x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
