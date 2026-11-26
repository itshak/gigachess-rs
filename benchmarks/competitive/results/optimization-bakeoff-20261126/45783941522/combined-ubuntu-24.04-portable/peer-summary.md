| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.91x | 1.52x | 1.35x | 1.87x | 1.33x | 30/30 |
| materialize | 0.99x | 2.00x | 1.78x | 1.26x | 2.35x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.90x | 1.57x | 1.21x | 1.95x | 1.69x | 30/30 |
| perft_full_d3 | 0.99x | 1.45x | 1.64x | 1.33x | 1.98x | 1.78x | 30/30 |
| fen_parse | 1.02x | 1.55x | 1.52x | 1.17x | 2.35x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 2.93x | 9.39x | 1.69x | 27.06x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.20x | 2.92x | 1.01x | 0.98x | 24/30 |
| san_parse_all | 0.96x | 2.03x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.05x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.94x | 1.33x | 3.88x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.29x | 63.19x | 105.85x | 187.60x | 114.91x | 5/5 |
| chess960_materialize | 0.98x | 1.50x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.08x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 27.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 21.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.85x | 1.92x | 1.26x | 2.10x | 2.53x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.61x | 1.17x | 2.08x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.22x | 3.22x | 2.45x | 3.10x | 1.75x | 10/10 |
| replay_trusted_native | 0.98x | 0.70x | 1.33x | 1.58x | 2.09x | 1.93x | 8/10 |
| replay_checked_native | 0.99x | 2.10x | 1.03x | 7.12x | 7.27x | 8.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
