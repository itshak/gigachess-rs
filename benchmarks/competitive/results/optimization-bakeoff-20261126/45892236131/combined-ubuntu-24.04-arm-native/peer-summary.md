| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.06x | 1.56x | 1.66x | 2.01x | 2.00x | 30/30 |
| materialize | 0.97x | 1.89x | 1.48x | 1.29x | 2.08x | 1.43x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.21x | 1.34x | 1.20x | 1.68x | 1.72x | 29/30 |
| perft_full_d3 | 0.75x | 1.11x | 1.21x | 1.00x | 1.46x | 1.40x | 25/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.77x | 1.38x | 30/30 |
| fen_write_native_ep | 0.94x | 4.26x | 8.84x | 1.51x | 28.17x | 2.26x | 30/30 |
| snapshot | 1.29x | 1.50x | 1.18x | 1.89x | 1.17x | 1.17x | 30/30 |
| san_parse_all | 0.95x | 1.75x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 0.80x | 9.91x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.83x | 1.03x | 1.21x | 3.35x | 1.00x | 1.01x | 21/30 |
| make_unmake_full | 0.74x | N/A | N/A | 1.15x | N/A | N/A | 5/6 |
| startpos | 1.07x | 1.31x | 81.85x | 136.64x | 185.15x | 147.08x | 5/5 |
| chess960_materialize | 0.97x | 1.65x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 2.58x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 26.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 21.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 4.04x | 1.58x | 1.30x | 1.83x | 2.03x | 10/10 |
| corpus_materialize | 0.99x | 1.57x | 1.52x | 1.22x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.16x | 2.99x | 2.13x | 2.90x | 1.49x | 10/10 |
| replay_trusted_native | 0.82x | 0.53x | 1.37x | 1.69x | 1.84x | 1.90x | 8/10 |
| replay_checked_native | 0.88x | 2.02x | 1.00x | 6.73x | 5.94x | 7.69x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
