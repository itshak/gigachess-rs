| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.70x | 1.71x | 1.63x | 2.51x | 2.06x | 30/30 |
| materialize | 1.00x | 1.95x | 1.69x | 1.26x | 2.31x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.04x | 1.61x | 1.40x | 2.42x | 2.01x | 30/30 |
| perft_full_d3 | 0.98x | 1.15x | 1.30x | 1.13x | 1.66x | 1.58x | 30/30 |
| fen_parse | 1.04x | 1.49x | 1.56x | 1.29x | 2.22x | 1.32x | 30/30 |
| fen_write_native_ep | 1.00x | 4.39x | 10.61x | 1.80x | 30.01x | 2.12x | 30/30 |
| snapshot | 1.02x | 1.99x | 1.25x | 3.55x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.03x | 2.09x | N/A | 4.51x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.36x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.30x | 1.32x | 4.60x | 1.48x | 1.91x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.39x | 147.90x | 229.95x | 145.72x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.83x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 20.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.51x | 1.86x | 1.39x | 2.81x | 2.43x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.53x | 1.19x | 2.01x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.55x | 3.39x | 2.51x | 3.24x | 1.85x | 10/10 |
| replay_trusted_native | 0.99x | 0.89x | 1.58x | 1.90x | 2.70x | 2.84x | 9/10 |
| replay_checked_native | 1.00x | 2.28x | 1.03x | 8.00x | 7.92x | 9.23x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
