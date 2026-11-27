| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.02x | 1.75x | 1.77x | 2.33x | 2.17x | 30/30 |
| materialize | 1.06x | 2.21x | 1.67x | 1.13x | 2.26x | 1.42x | 29/30 |
| perft_bulk_d4 | 1.04x | 4.32x | 1.68x | 1.33x | 2.13x | 1.86x | 30/30 |
| perft_full_d3 | 1.06x | 1.69x | 1.89x | 1.46x | 2.10x | 2.12x | 30/30 |
| fen_parse | 0.98x | 1.16x | 1.22x | 1.32x | 1.65x | 1.45x | 28/30 |
| fen_write_native_ep | 0.91x | 3.53x | 7.65x | 1.57x | 22.98x | 1.92x | 30/30 |
| snapshot | 1.12x | 1.48x | 1.07x | 2.37x | 1.11x | 0.96x | 24/30 |
| san_parse_all | 1.01x | 1.80x | N/A | 3.73x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 12.39x | N/A | 2.77x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.17x | 1.25x | 5.77x | 1.20x | 1.41x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.76x | 82.86x | 168.61x | 173.64x | 195.36x | 5/5 |
| chess960_materialize | 0.93x | 2.00x | 1.92x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.72x | 3.30x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.68x | 37.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.15x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 22.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 7.47x | 2.55x | 1.66x | 3.27x | 3.43x | 10/10 |
| corpus_materialize | 0.76x | 1.46x | 1.42x | 0.97x | 1.73x | 1.11x | 9/10 |
| corpus_uci_sorted | 0.73x | 1.85x | 2.49x | 1.88x | 2.52x | 1.33x | 10/10 |
| replay_trusted_native | 0.70x | 0.54x | 1.48x | 1.55x | 1.63x | 1.68x | 8/10 |
| replay_checked_native | 0.81x | 2.35x | 1.08x | 8.70x | 7.69x | 9.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
