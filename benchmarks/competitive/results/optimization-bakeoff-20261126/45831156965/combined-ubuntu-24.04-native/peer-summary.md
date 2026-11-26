| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.88x | 1.66x | 1.42x | 1.98x | 1.72x | 30/30 |
| materialize | 1.02x | 1.90x | 1.50x | 1.54x | 2.20x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.28x | 1.33x | 1.01x | 1.86x | 1.68x | 26/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.79x | 1.61x | 2.10x | 1.83x | 30/30 |
| fen_parse | 1.09x | 1.24x | 1.41x | 1.08x | 1.60x | 1.20x | 30/30 |
| fen_write_native_ep | 0.94x | 4.52x | 9.75x | 2.01x | 28.78x | 2.50x | 30/30 |
| snapshot | 1.00x | 1.84x | 1.16x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.97x | 1.80x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.28x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.07x | 1.32x | 4.07x | 1.30x | 1.64x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 90.84x | 148.50x | 209.68x | 160.83x | 5/5 |
| chess960_materialize | 1.00x | 1.43x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.35x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 23.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.16x | 1.62x | 0.84x | 1.86x | 1.76x | 8/10 |
| corpus_materialize | 1.00x | 1.35x | 1.25x | 1.03x | 1.64x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 3.01x | 2.24x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.57x | 1.55x | 2.18x | 2.50x | 9/10 |
| replay_checked_native | 1.00x | 2.16x | 1.12x | 8.50x | 8.28x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
