| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.58x | 1.92x | 1.67x | 2.26x | 2.02x | 30/30 |
| materialize | 0.99x | 2.04x | 1.41x | 1.60x | 2.40x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.07x | 1.64x | 1.26x | 2.25x | 1.96x | 29/30 |
| perft_full_d3 | 1.21x | 1.71x | 1.94x | 1.76x | 2.44x | 2.01x | 30/30 |
| fen_parse | 1.13x | 1.15x | 1.45x | 1.28x | 1.72x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.25x | 8.32x | 1.68x | 30.33x | 2.20x | 30/30 |
| snapshot | 0.91x | 2.10x | 1.15x | 4.95x | 1.14x | 1.18x | 30/30 |
| san_parse_all | 1.08x | 1.77x | N/A | 5.35x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.70x | N/A | 4.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.10x | 1.36x | 10.60x | 1.34x | 1.67x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.98x | 0.99x | 77.21x | 155.78x | 206.08x | 165.91x | 4/5 |
| chess960_materialize | 0.98x | 1.58x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.61x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.57x | 1.60x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 33.60x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 5.63x | 2.28x | 1.13x | 2.53x | 2.44x | 10/10 |
| corpus_materialize | 1.05x | 1.42x | 1.23x | 1.09x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.84x | 2.21x | 1.89x | 2.20x | 1.55x | 10/10 |
| replay_trusted_native | 1.06x | 0.85x | 1.67x | 1.70x | 2.25x | 2.59x | 9/10 |
| replay_checked_native | 0.99x | 2.25x | 1.17x | 9.06x | 8.75x | 10.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
