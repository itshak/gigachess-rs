| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.08x | 1.44x | 1.41x | 2.12x | 1.75x | 30/30 |
| materialize | 1.00x | 1.79x | 1.53x | 1.18x | 2.08x | 1.43x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.65x | 1.47x | 1.24x | 2.28x | 1.84x | 30/30 |
| perft_full_d3 | 0.99x | 0.83x | 0.93x | 0.80x | 1.31x | 1.16x | 13/30 |
| fen_parse | 0.94x | 1.16x | 1.30x | 1.04x | 1.88x | 1.13x | 27/30 |
| fen_write_native_ep | 1.00x | 2.35x | 5.80x | 0.95x | 15.87x | 1.11x | 25/30 |
| snapshot | 1.32x | 1.57x | 1.19x | 2.48x | 1.19x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 1.89x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.81x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.56x | 0.91x | 3.02x | 0.99x | 1.11x | 20/30 |
| make_unmake_full | 1.02x | N/A | N/A | 0.99x | N/A | N/A | 4/6 |
| startpos | 1.08x | 1.20x | 72.45x | 132.75x | 210.48x | 141.77x | 5/5 |
| chess960_materialize | 0.98x | 1.37x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.37x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.25x | 1.75x | 1.31x | 2.56x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.57x | 1.15x | 2.01x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.72x | 2.38x | 3.17x | 2.36x | 3.11x | 1.66x | 10/10 |
| replay_trusted_native | 0.98x | 0.59x | 1.17x | 1.32x | 1.76x | 2.05x | 8/10 |
| replay_checked_native | 0.99x | 2.10x | 0.96x | 6.89x | 6.91x | 8.50x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
