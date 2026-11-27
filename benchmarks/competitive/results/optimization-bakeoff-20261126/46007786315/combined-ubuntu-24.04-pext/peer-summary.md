| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.33x | 1.80x | 1.82x | 2.86x | 2.11x | 30/30 |
| materialize | 1.03x | 2.27x | 1.83x | 1.21x | 2.45x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.48x | 1.86x | 1.43x | 2.75x | 2.23x | 30/30 |
| perft_full_d3 | 1.33x | 2.36x | 2.87x | 1.60x | 3.05x | 2.41x | 30/30 |
| fen_parse | 0.99x | 1.40x | 1.53x | 1.17x | 1.96x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.08x | 9.54x | 1.84x | 28.22x | 2.11x | 30/30 |
| snapshot | 1.91x | 1.89x | 1.25x | 3.82x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.27x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.33x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.96x | 1.28x | 4.64x | 1.48x | 1.69x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 100.04x | 136.00x | 207.48x | 160.63x | 5/5 |
| chess960_materialize | 1.00x | 1.91x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.35x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.08x | 1.91x | 1.46x | 2.67x | 2.19x | 10/10 |
| corpus_materialize | 1.02x | 2.01x | 1.83x | 1.27x | 2.92x | 1.58x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.38x | 3.20x | 2.38x | 2.95x | 1.70x | 10/10 |
| replay_trusted_native | 0.99x | 1.19x | 2.08x | 2.34x | 3.95x | 4.72x | 10/10 |
| replay_checked_native | 0.99x | 2.85x | 1.12x | 6.25x | 7.03x | 8.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
