| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.25x | 1.76x | 1.62x | 2.19x | 1.86x | 30/30 |
| materialize | 0.99x | 2.17x | 1.44x | 1.64x | 2.64x | 1.75x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.65x | 1.57x | 1.11x | 2.02x | 1.73x | 28/30 |
| perft_full_d3 | 0.93x | 1.44x | 1.50x | 1.42x | 2.02x | 1.66x | 30/30 |
| fen_parse | 0.92x | 1.28x | 1.60x | 1.32x | 1.69x | 1.29x | 30/30 |
| fen_write_native_ep | 0.95x | 4.61x | 8.64x | 1.73x | 32.33x | 2.27x | 30/30 |
| snapshot | 0.90x | 2.06x | 1.10x | 4.77x | 1.08x | 1.13x | 27/30 |
| san_parse_all | 0.91x | 1.93x | N/A | 5.24x | N/A | N/A | 12/12 |
| san_render_native | 0.88x | 9.90x | N/A | 4.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.93x | 1.31x | 10.73x | 1.30x | 1.55x | 30/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.00x | 77.51x | 158.10x | 207.28x | 159.88x | 5/5 |
| chess960_materialize | 1.02x | 1.60x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.53x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.28x | 33.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.68x | 1.86x | 0.93x | 2.10x | 2.02x | 8/10 |
| corpus_materialize | 1.02x | 1.41x | 1.20x | 1.08x | 1.75x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.82x | 2.25x | 1.88x | 2.17x | 1.54x | 10/10 |
| replay_trusted_native | 1.10x | 0.87x | 1.59x | 1.68x | 2.33x | 2.66x | 9/10 |
| replay_checked_native | 1.01x | 2.30x | 1.20x | 9.33x | 9.11x | 10.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
