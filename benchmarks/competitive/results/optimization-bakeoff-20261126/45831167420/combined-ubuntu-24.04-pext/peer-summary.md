| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.42x | 1.71x | 1.69x | 2.33x | 1.89x | 30/30 |
| materialize | 1.05x | 2.14x | 1.58x | 1.44x | 2.38x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.13x | 3.60x | 1.42x | 1.23x | 2.00x | 1.68x | 29/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.50x | 1.33x | 1.83x | 1.71x | 30/30 |
| fen_parse | 0.93x | 1.22x | 1.43x | 1.13x | 1.65x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 5.44x | 10.62x | 1.98x | 30.24x | 2.61x | 30/30 |
| snapshot | 0.99x | 2.04x | 1.12x | 4.62x | 1.86x | 1.32x | 30/30 |
| san_parse_all | 1.01x | 1.98x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.09x | N/A | 2.45x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.10x | 1.29x | 4.55x | 1.32x | 1.49x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 90.90x | 166.10x | 223.80x | 165.79x | 5/5 |
| chess960_materialize | 1.08x | 1.74x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.48x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 31.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 20.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.61x | 1.73x | 1.06x | 2.10x | 1.98x | 10/10 |
| corpus_materialize | 1.06x | 1.54x | 1.43x | 1.15x | 1.82x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.21x | 2.86x | 2.21x | 2.78x | 1.49x | 10/10 |
| replay_trusted_native | 1.01x | 0.86x | 1.56x | 1.81x | 2.23x | 2.34x | 9/10 |
| replay_checked_native | 1.01x | 2.18x | 1.07x | 8.16x | 7.51x | 8.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
