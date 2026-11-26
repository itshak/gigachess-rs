| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.26x | 1.83x | 1.54x | 2.34x | 1.87x | 30/30 |
| materialize | 0.94x | 2.09x | 1.58x | 1.70x | 2.55x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.59x | 1.47x | 1.13x | 2.18x | 1.77x | 29/30 |
| perft_full_d3 | 1.00x | 1.28x | 1.53x | 1.41x | 1.82x | 2.15x | 30/30 |
| fen_parse | 1.00x | 1.46x | 1.35x | 1.13x | 1.46x | 1.13x | 30/30 |
| fen_write_native_ep | 0.97x | 4.80x | 10.04x | 1.99x | 27.14x | 2.43x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 1.19x | 8.91x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.84x | 1.17x | 3.53x | 1.10x | 1.51x | 29/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.07x | N/A | N/A | 5/6 |
| startpos | 1.26x | 1.25x | 99.92x | 179.76x | 226.41x | 178.83x | 5/5 |
| chess960_materialize | 0.94x | 1.67x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.39x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 37.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.91x | 22.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.26x | 1.99x | 1.09x | 2.37x | 2.21x | 9/10 |
| corpus_materialize | 0.99x | 1.38x | 1.29x | 1.04x | 1.71x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 2.98x | 2.22x | 2.95x | 1.56x | 10/10 |
| replay_trusted_native | 1.15x | 0.99x | 1.82x | 1.80x | 2.48x | 2.93x | 9/10 |
| replay_checked_native | 1.01x | 2.02x | 1.06x | 8.40x | 8.10x | 9.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
