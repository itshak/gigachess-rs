| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 3.11x | 1.58x | 1.45x | 1.95x | 1.39x | 30/30 |
| materialize | 0.98x | 2.19x | 1.75x | 1.26x | 2.29x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.12x | 1.62x | 1.28x | 2.03x | 1.73x | 30/30 |
| perft_full_d3 | 0.98x | 1.27x | 1.49x | 1.15x | 1.76x | 1.64x | 29/30 |
| fen_parse | 1.01x | 1.66x | 1.50x | 1.26x | 1.93x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 2.65x | 9.41x | 1.70x | 26.08x | 1.97x | 30/30 |
| snapshot | 1.04x | 1.72x | 1.21x | 2.83x | 1.14x | 0.89x | 24/30 |
| san_parse_all | 1.00x | 2.15x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.70x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.04x | 1.27x | 4.88x | 1.20x | 1.24x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 73.14x | 109.88x | 158.54x | 104.87x | 5/5 |
| chess960_materialize | 1.03x | 1.73x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.32x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.22x | 1.80x | 1.26x | 1.97x | 1.95x | 10/10 |
| corpus_materialize | 1.03x | 2.01x | 1.83x | 1.34x | 3.01x | 1.76x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.05x | 2.96x | 2.24x | 2.83x | 1.61x | 10/10 |
| replay_trusted_native | 1.01x | 0.75x | 1.52x | 1.66x | 2.40x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.22x | 0.97x | 5.34x | 6.17x | 7.10x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
