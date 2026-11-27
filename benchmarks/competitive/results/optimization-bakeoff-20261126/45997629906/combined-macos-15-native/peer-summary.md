| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.98x | 1.65x | 1.78x | 2.43x | 2.00x | 30/30 |
| materialize | 1.02x | 2.03x | 1.58x | 1.17x | 2.28x | 1.42x | 27/30 |
| perft_bulk_d4 | 1.02x | 4.20x | 1.63x | 1.24x | 1.99x | 1.92x | 29/30 |
| perft_full_d3 | 0.90x | 1.46x | 1.54x | 1.12x | 1.69x | 1.72x | 29/30 |
| fen_parse | 1.11x | 1.25x | 1.39x | 1.41x | 1.73x | 1.82x | 30/30 |
| fen_write_native_ep | 1.08x | 4.30x | 9.07x | 1.75x | 26.88x | 2.32x | 30/30 |
| snapshot | 1.08x | 1.86x | 1.22x | 2.58x | 1.17x | 1.05x | 28/30 |
| san_parse_all | 0.98x | 1.71x | N/A | 3.67x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.56x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.23x | 1.26x | 6.40x | 1.30x | 1.49x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.22x | 76.19x | 137.17x | 169.75x | 165.87x | 5/5 |
| chess960_materialize | 0.97x | 1.72x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.44x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 34.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.17x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.95x | 1.90x | 1.17x | 2.37x | 2.48x | 9/10 |
| corpus_materialize | 1.14x | 1.53x | 1.58x | 1.22x | 2.10x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.21x | 1.92x | 2.72x | 1.94x | 2.55x | 1.65x | 10/10 |
| replay_trusted_native | 1.17x | 0.62x | 1.59x | 1.63x | 1.85x | 1.74x | 8/10 |
| replay_checked_native | 1.13x | 2.69x | 1.22x | 9.46x | 8.43x | 10.43x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
