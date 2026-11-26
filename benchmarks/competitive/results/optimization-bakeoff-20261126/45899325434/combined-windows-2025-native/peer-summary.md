| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.74x | 1.76x | 1.69x | 2.64x | 2.13x | 30/30 |
| materialize | 1.00x | 2.03x | 1.38x | 1.16x | 2.34x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.11x | 1.68x | 1.48x | 2.55x | 2.31x | 30/30 |
| perft_full_d3 | 1.32x | 1.97x | 2.05x | 1.70x | 2.77x | 2.48x | 30/30 |
| fen_parse | 1.00x | 1.45x | 1.51x | 1.44x | 2.40x | 1.62x | 30/30 |
| fen_write_native_ep | 1.04x | 4.05x | 7.74x | 1.63x | 33.40x | 2.01x | 30/30 |
| snapshot | 1.02x | 1.94x | 1.29x | 4.13x | 1.27x | 1.10x | 29/30 |
| san_parse_all | 1.00x | 1.99x | N/A | 4.82x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.36x | N/A | 3.48x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.95x | 1.31x | 8.81x | 1.55x | 1.75x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 84.57x | 134.61x | 211.38x | 153.05x | 4/5 |
| chess960_materialize | 1.01x | 1.66x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.90x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.22x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.58x | 1.85x | 1.36x | 2.48x | 2.23x | 10/10 |
| corpus_materialize | 1.00x | 1.83x | 1.39x | 1.20x | 2.85x | 1.53x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.88x | 2.40x | 1.97x | 2.29x | 1.50x | 10/10 |
| replay_trusted_native | 1.06x | 1.19x | 2.06x | 2.56x | 4.12x | 5.04x | 10/10 |
| replay_checked_native | 1.01x | 2.48x | 1.08x | 6.13x | 7.54x | 8.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
