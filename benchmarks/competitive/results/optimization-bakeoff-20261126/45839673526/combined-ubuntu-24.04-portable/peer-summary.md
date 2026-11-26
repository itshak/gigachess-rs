| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.82x | 1.61x | 1.40x | 1.85x | 1.35x | 29/30 |
| materialize | 0.92x | 2.19x | 1.71x | 1.85x | 2.53x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.95x | 2.85x | 1.53x | 1.13x | 1.88x | 1.69x | 29/30 |
| perft_full_d3 | 0.93x | 1.22x | 1.42x | 1.26x | 1.67x | 1.41x | 30/30 |
| fen_parse | 0.92x | 1.61x | 1.31x | 1.09x | 1.41x | 1.12x | 30/30 |
| fen_write_native_ep | 1.00x | 3.08x | 9.95x | 1.80x | 27.66x | 2.36x | 30/30 |
| snapshot | 1.00x | 1.74x | 1.24x | 3.23x | 1.24x | 0.99x | 24/30 |
| san_parse_all | 1.03x | 1.88x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 8.51x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.09x | 1.35x | 3.93x | 1.25x | 1.49x | 30/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 63.14x | 108.16x | 160.24x | 106.24x | 5/5 |
| chess960_materialize | 0.91x | 1.65x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 1.96x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 19.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.00x | 2.02x | 1.10x | 2.16x | 2.75x | 10/10 |
| corpus_materialize | 0.99x | 1.48x | 1.39x | 1.12x | 1.82x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.90x | 3.02x | 2.19x | 2.87x | 1.53x | 10/10 |
| replay_trusted_native | 0.98x | 0.86x | 1.64x | 1.60x | 2.20x | 2.03x | 9/10 |
| replay_checked_native | 0.96x | 1.91x | 1.07x | 8.04x | 7.81x | 8.95x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
