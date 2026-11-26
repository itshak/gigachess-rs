| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.78x | 1.45x | 1.67x | 2.10x | 1.83x | 29/30 |
| materialize | 0.92x | 1.97x | 1.45x | 0.98x | 2.18x | 1.27x | 24/30 |
| perft_bulk_d4 | 0.99x | 4.16x | 1.53x | 1.30x | 1.97x | 2.02x | 30/30 |
| perft_full_d3 | 1.02x | 1.51x | 1.74x | 1.40x | 2.01x | 2.04x | 30/30 |
| fen_parse | 0.98x | 1.20x | 1.34x | 1.39x | 1.67x | 1.57x | 30/30 |
| fen_write_native_ep | 0.99x | 4.00x | 8.75x | 1.79x | 29.31x | 2.46x | 30/30 |
| snapshot | 0.98x | 1.55x | 1.13x | 2.15x | 1.14x | 0.92x | 21/30 |
| san_parse_all | 0.94x | 1.92x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 0.88x | 10.55x | N/A | 2.50x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.10x | 1.12x | 5.58x | 1.09x | 1.23x | 26/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.83x | 78.43x | 156.64x | 197.57x | 181.22x | 5/5 |
| chess960_materialize | 1.31x | 1.58x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.17x | 2.98x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.18x | 38.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 15.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.97x | 2.45x | 1.61x | 2.70x | 2.94x | 10/10 |
| corpus_materialize | 1.24x | 1.62x | 1.69x | 1.17x | 2.00x | 1.30x | 10/10 |
| corpus_uci_sorted | 0.91x | 1.70x | 2.56x | 1.75x | 2.34x | 1.30x | 10/10 |
| replay_trusted_native | 1.20x | 0.57x | 1.58x | 1.64x | 1.63x | 1.67x | 8/10 |
| replay_checked_native | 1.15x | 2.07x | 1.02x | 7.70x | 6.65x | 7.78x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
