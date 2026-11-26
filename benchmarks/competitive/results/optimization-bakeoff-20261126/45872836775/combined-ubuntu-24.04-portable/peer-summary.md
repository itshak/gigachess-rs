| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.16x | 1.60x | 1.50x | 2.05x | 1.44x | 30/30 |
| materialize | 0.99x | 2.12x | 1.87x | 1.34x | 2.49x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.08x | 1.67x | 1.30x | 2.15x | 1.80x | 30/30 |
| perft_full_d3 | 1.00x | 1.39x | 1.58x | 1.28x | 1.90x | 1.73x | 30/30 |
| fen_parse | 1.05x | 1.39x | 1.44x | 1.25x | 2.08x | 1.21x | 30/30 |
| fen_write_native_ep | 1.01x | 2.86x | 9.07x | 1.61x | 26.23x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.08x | 3.05x | 1.00x | 1.07x | 26/30 |
| san_parse_all | 0.99x | 2.02x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.97x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.24x | 1.56x | 4.57x | 1.44x | 1.48x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 64.25x | 120.75x | 182.16x | 113.63x | 5/5 |
| chess960_materialize | 0.97x | 1.64x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.27x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 21.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.74x | 0.75x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.01x | 17.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.83x | 1.91x | 1.28x | 2.13x | 2.57x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.61x | 1.20x | 2.09x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.22x | 3.20x | 2.45x | 3.08x | 1.82x | 10/10 |
| replay_trusted_native | 0.99x | 0.90x | 1.72x | 2.03x | 2.69x | 2.46x | 9/10 |
| replay_checked_native | 1.02x | 2.33x | 1.15x | 8.34x | 8.38x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
