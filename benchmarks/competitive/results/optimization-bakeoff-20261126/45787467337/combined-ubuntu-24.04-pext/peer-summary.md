| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.32x | 1.79x | 1.55x | 2.52x | 1.84x | 30/30 |
| materialize | 1.00x | 2.20x | 1.70x | 1.71x | 2.85x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.07x | 1.61x | 1.26x | 2.58x | 1.98x | 30/30 |
| perft_full_d3 | 0.94x | 1.50x | 1.66x | 1.62x | 2.22x | 1.76x | 30/30 |
| fen_parse | 0.95x | 1.23x | 1.45x | 0.97x | 1.32x | 1.13x | 25/30 |
| fen_write_native_ep | 1.00x | 4.60x | 10.14x | 1.93x | 29.16x | 2.60x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.16x | 1.92x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.19x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.72x | 1.08x | 3.27x | 1.09x | 1.27x | 29/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.01x | 79.12x | 136.52x | 166.95x | 151.40x | 5/5 |
| chess960_materialize | 1.06x | 1.76x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.38x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 41.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 22.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.82x | 2.53x | 1.42x | 3.11x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.46x | 1.33x | 1.12x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 2.99x | 2.26x | 2.99x | 1.59x | 10/10 |
| replay_trusted_native | 1.04x | 0.74x | 1.34x | 1.31x | 1.87x | 2.18x | 8/10 |
| replay_checked_native | 1.02x | 2.12x | 1.09x | 8.56x | 8.39x | 9.79x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
