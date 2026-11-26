| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.16x | 1.62x | 1.46x | 2.04x | 1.41x | 30/30 |
| materialize | 1.04x | 2.44x | 1.92x | 1.35x | 2.58x | 1.72x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.09x | 1.61x | 1.28x | 2.02x | 1.73x | 30/30 |
| perft_full_d3 | 1.01x | 1.43x | 1.69x | 1.30x | 1.99x | 1.88x | 30/30 |
| fen_parse | 0.88x | 1.43x | 1.33x | 1.03x | 1.74x | 1.17x | 28/30 |
| fen_write_native_ep | 0.99x | 2.57x | 9.20x | 1.75x | 25.74x | 1.92x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.83x | 1.14x | 0.98x | 24/30 |
| san_parse_all | 0.98x | 2.19x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.85x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 2.41x | 1.48x | 5.69x | 1.42x | 1.45x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 72.91x | 102.10x | 164.96x | 113.27x | 5/5 |
| chess960_materialize | 1.09x | 1.94x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.41x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.19x | 1.78x | 1.25x | 1.93x | 1.85x | 10/10 |
| corpus_materialize | 1.00x | 2.08x | 1.84x | 1.38x | 3.02x | 1.76x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.06x | 2.97x | 2.23x | 2.84x | 1.59x | 10/10 |
| replay_trusted_native | 1.19x | 1.13x | 2.27x | 2.49x | 3.61x | 3.32x | 10/10 |
| replay_checked_native | 1.09x | 2.64x | 1.16x | 6.28x | 6.72x | 8.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
