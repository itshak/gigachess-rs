| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.51x | 1.90x | 1.63x | 2.30x | 1.99x | 30/30 |
| materialize | 1.02x | 2.07x | 1.66x | 1.67x | 2.43x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.02x | 1.63x | 1.24x | 2.26x | 1.94x | 29/30 |
| perft_full_d3 | 1.25x | 2.04x | 1.98x | 1.86x | 2.41x | 2.11x | 30/30 |
| fen_parse | 0.91x | 1.28x | 1.34x | 1.17x | 1.52x | 1.26x | 30/30 |
| fen_write_native_ep | 0.94x | 4.69x | 10.03x | 1.94x | 30.09x | 2.46x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.04x | 1.84x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.42x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.36x | 4.19x | 1.35x | 1.71x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 90.87x | 165.59x | 205.72x | 176.87x | 5/5 |
| chess960_materialize | 1.04x | 1.63x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.57x | 1.62x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 26.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.26x | 5.86x | 2.32x | 1.19x | 2.64x | 2.52x | 10/10 |
| corpus_materialize | 1.01x | 1.38x | 1.30x | 1.06x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.01x | 2.22x | 2.88x | 1.53x | 10/10 |
| replay_trusted_native | 1.02x | 0.86x | 1.55x | 1.57x | 2.17x | 2.51x | 9/10 |
| replay_checked_native | 1.00x | 2.36x | 1.21x | 9.21x | 8.90x | 10.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
