| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 5.06x | 1.70x | 1.78x | 2.27x | 2.09x | 30/30 |
| materialize | 1.03x | 2.10x | 1.59x | 1.08x | 2.54x | 1.43x | 27/30 |
| perft_bulk_d4 | 1.08x | 4.06x | 1.50x | 1.22x | 1.89x | 1.82x | 29/30 |
| perft_full_d3 | 0.91x | 1.56x | 1.63x | 1.25x | 1.83x | 1.87x | 28/30 |
| fen_parse | 0.99x | 1.30x | 1.39x | 1.42x | 1.71x | 1.44x | 30/30 |
| fen_write_native_ep | 0.99x | 4.15x | 8.98x | 1.76x | 28.14x | 2.25x | 30/30 |
| snapshot | 0.99x | 1.69x | 1.13x | 2.43x | 1.13x | 1.01x | 27/30 |
| san_parse_all | 1.01x | 1.77x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.39x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.13x | 1.20x | 5.86x | 1.17x | 1.32x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.41x | 1.19x | 67.50x | 133.70x | 158.78x | 129.41x | 5/5 |
| chess960_materialize | 1.10x | 1.80x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.25x | 3.29x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 38.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 19.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.20x | 5.66x | 1.89x | 1.13x | 2.20x | 2.38x | 9/10 |
| corpus_materialize | 1.12x | 1.49x | 1.39x | 1.13x | 1.96x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.86x | 2.66x | 1.91x | 2.54x | 1.41x | 10/10 |
| replay_trusted_native | 1.01x | 0.65x | 1.66x | 1.67x | 1.79x | 1.79x | 8/10 |
| replay_checked_native | 0.98x | 2.45x | 1.16x | 8.68x | 7.86x | 9.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
