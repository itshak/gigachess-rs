| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.90x | 1.81x | 1.74x | 2.72x | 2.20x | 30/30 |
| materialize | 1.01x | 2.05x | 1.39x | 1.15x | 2.38x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.19x | 1.72x | 1.53x | 2.61x | 2.36x | 30/30 |
| perft_full_d3 | 1.33x | 1.97x | 2.08x | 1.70x | 2.78x | 2.45x | 30/30 |
| fen_parse | 1.07x | 1.52x | 1.67x | 1.60x | 2.27x | 1.59x | 30/30 |
| fen_write_native_ep | 0.99x | 3.98x | 7.89x | 1.57x | 31.23x | 2.00x | 30/30 |
| snapshot | 1.00x | 1.80x | 1.25x | 4.12x | 1.27x | 1.13x | 30/30 |
| san_parse_all | 1.02x | 1.96x | N/A | 4.79x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.34x | N/A | 3.42x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.95x | 1.33x | 8.41x | 1.54x | 1.75x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 85.91x | 138.63x | 196.85x | 144.44x | 4/5 |
| chess960_materialize | 1.00x | 1.65x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.90x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.69x | 1.89x | 1.38x | 2.54x | 2.28x | 10/10 |
| corpus_materialize | 1.01x | 1.83x | 1.43x | 1.21x | 2.87x | 1.53x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.89x | 2.42x | 2.00x | 2.32x | 1.51x | 10/10 |
| replay_trusted_native | 1.13x | 1.20x | 2.12x | 2.58x | 4.10x | 5.08x | 10/10 |
| replay_checked_native | 0.99x | 2.47x | 1.10x | 6.08x | 7.34x | 8.30x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
