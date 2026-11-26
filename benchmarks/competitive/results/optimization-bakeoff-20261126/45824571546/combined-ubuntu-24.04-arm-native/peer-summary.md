| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.82x | 1.45x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.38x | 1.26x | 1.77x | 1.82x | 30/30 |
| perft_full_d3 | 1.01x | 1.20x | 1.30x | 1.08x | 1.59x | 1.50x | 29/30 |
| fen_parse | 1.01x | 1.25x | 1.34x | 1.31x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.23x | 1.60x | 30.03x | 2.39x | 30/30 |
| snapshot | 1.02x | 1.15x | 1.01x | 1.47x | 1.01x | 0.94x | 22/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.84x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.39x | 1.63x | 4.51x | 1.35x | 1.34x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.37x | 129.39x | 171.78x | 136.80x | 5/5 |
| chess960_materialize | 0.99x | 1.56x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.67x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 26.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.93x | 1.48x | 1.19x | 1.76x | 1.94x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.44x | 1.15x | 1.82x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 0.98x | 0.60x | 1.58x | 1.93x | 2.14x | 2.18x | 8/10 |
| replay_checked_native | 0.99x | 2.18x | 1.05x | 7.04x | 6.32x | 8.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
