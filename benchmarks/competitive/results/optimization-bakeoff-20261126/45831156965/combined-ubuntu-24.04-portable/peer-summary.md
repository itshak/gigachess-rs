| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.74x | 1.56x | 1.35x | 1.81x | 1.29x | 29/30 |
| materialize | 1.00x | 2.16x | 1.73x | 1.80x | 2.66x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.70x | 1.43x | 1.07x | 1.79x | 1.60x | 29/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.76x | 1.54x | 2.10x | 1.74x | 30/30 |
| fen_parse | 1.01x | 1.43x | 1.34x | 1.05x | 1.43x | 1.08x | 30/30 |
| fen_write_native_ep | 0.98x | 2.92x | 10.08x | 2.10x | 28.85x | 2.20x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.95x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.24x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.42x | 4.06x | 1.33x | 1.53x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 56.89x | 106.73x | 126.73x | 97.50x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 1.98x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.61x | 1.78x | 0.98x | 1.88x | 2.46x | 9/10 |
| corpus_materialize | 1.00x | 1.44x | 1.34x | 1.08x | 1.77x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.94x | 2.96x | 2.16x | 2.87x | 1.53x | 10/10 |
| replay_trusted_native | 1.01x | 0.87x | 1.69x | 1.63x | 2.27x | 2.04x | 9/10 |
| replay_checked_native | 1.02x | 1.94x | 1.09x | 8.36x | 8.12x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
