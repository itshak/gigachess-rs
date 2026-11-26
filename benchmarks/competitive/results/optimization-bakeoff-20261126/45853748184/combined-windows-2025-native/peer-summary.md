| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.88x | 1.68x | 1.63x | 2.41x | 2.13x | 30/30 |
| materialize | 1.02x | 2.05x | 1.34x | 1.25x | 2.33x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.14x | 1.61x | 1.35x | 2.35x | 2.07x | 30/30 |
| perft_full_d3 | 1.03x | 1.43x | 1.52x | 1.33x | 1.97x | 1.88x | 30/30 |
| fen_parse | 1.06x | 1.33x | 1.40x | 1.32x | 2.20x | 1.53x | 30/30 |
| fen_write_native_ep | 0.98x | 4.16x | 7.78x | 1.51x | 30.35x | 1.91x | 30/30 |
| snapshot | 1.00x | 1.84x | 1.00x | 3.98x | 1.01x | 1.07x | 26/30 |
| san_parse_all | 1.00x | 2.07x | N/A | 5.81x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.13x | N/A | 3.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.20x | 1.35x | 9.24x | 1.41x | 1.72x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 79.13x | 158.47x | 231.53x | 176.85x | 5/5 |
| chess960_materialize | 1.01x | 1.64x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 25.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.72x | 1.83x | 1.43x | 2.44x | 2.56x | 10/10 |
| corpus_materialize | 0.97x | 1.57x | 1.23x | 1.15x | 1.99x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.78x | 2.16x | 1.79x | 2.15x | 1.43x | 10/10 |
| replay_trusted_native | 0.96x | 0.80x | 1.58x | 1.99x | 2.51x | 2.85x | 8/10 |
| replay_checked_native | 0.89x | 2.08x | 0.94x | 7.23x | 6.98x | 8.37x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
