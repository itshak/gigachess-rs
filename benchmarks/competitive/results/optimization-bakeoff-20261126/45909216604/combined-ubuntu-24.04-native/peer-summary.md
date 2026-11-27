| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.96x | 1.72x | 1.63x | 2.37x | 2.07x | 30/30 |
| materialize | 0.98x | 2.13x | 1.68x | 1.24x | 2.36x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.30x | 1.64x | 1.30x | 2.47x | 2.35x | 30/30 |
| perft_full_d3 | 0.99x | 1.39x | 1.65x | 1.27x | 2.08x | 2.06x | 30/30 |
| fen_parse | 0.89x | 1.26x | 1.41x | 1.12x | 1.90x | 1.26x | 30/30 |
| fen_write_native_ep | 0.99x | 4.12x | 9.10x | 1.54x | 25.53x | 1.91x | 30/30 |
| snapshot | 0.99x | 2.49x | 1.50x | 4.48x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.79x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.17x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.91x | 1.43x | 4.02x | 1.36x | 1.93x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.26x | 85.87x | 142.03x | 222.19x | 154.32x | 5/5 |
| chess960_materialize | 0.98x | 1.68x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.92x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 36.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 28.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.15x | 1.85x | 1.23x | 2.43x | 2.27x | 9/10 |
| corpus_materialize | 1.00x | 1.66x | 1.58x | 1.13x | 2.19x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.36x | 3.11x | 2.39x | 3.06x | 1.65x | 10/10 |
| replay_trusted_native | 1.02x | 0.77x | 1.42x | 1.59x | 2.53x | 3.05x | 9/10 |
| replay_checked_native | 1.02x | 2.63x | 1.21x | 9.09x | 9.27x | 11.71x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
