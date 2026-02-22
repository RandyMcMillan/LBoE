use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame, Terminal,
};
use std::io;

// ── Embedded text files ────────────────────────────────────────────────────────

static INTRO: &[&[u8]] = &[
    include_bytes!("../intro_p0.txt"),
    include_bytes!("../intro_p1.txt"),
    include_bytes!("../intro_p2.txt"),
    include_bytes!("../intro_p3.txt"),
    include_bytes!("../intro_p4.txt"),
    include_bytes!("../intro_p5.txt"),
    include_bytes!("../intro_p6.txt"),
    include_bytes!("../intro_p7.txt"),
    include_bytes!("../intro_p8.txt"),
    include_bytes!("../intro_p9.txt"),
];

static TABLET1: &[&[u8]] = &[
    include_bytes!("../tablet1_p10.txt"),
    include_bytes!("../tablet1_p11.txt"),
    include_bytes!("../tablet1_p12.txt"),
    include_bytes!("../tablet1_p13.txt"),
    include_bytes!("../tablet1_p14.txt"),
    include_bytes!("../tablet1_p15.txt"),
    include_bytes!("../tablet1_p16.txt"),
    include_bytes!("../tablet1_p17.txt"),
    include_bytes!("../tablet1_p18.txt"),
    include_bytes!("../tablet1_p19.txt"),
    include_bytes!("../tablet1_p20.txt"),
    include_bytes!("../tablet1_p21.txt"),
    include_bytes!("../tablet1_p22.txt"),
    include_bytes!("../tablet1_p23.txt"),
    include_bytes!("../tablet1_p24.txt"),
    include_bytes!("../tablet1_p25.txt"),
    include_bytes!("../tablet1_p26.txt"),
];

static TABLET2: &[&[u8]] = &[
    include_bytes!("../tablet2_p27.txt"),
    include_bytes!("../tablet2_p28.txt"),
    include_bytes!("../tablet2_p29.txt"),
    include_bytes!("../tablet2_p30.txt"),
    include_bytes!("../tablet2_p31.txt"),
    include_bytes!("../tablet2_p32.txt"),
    include_bytes!("../tablet2_p33.txt"),
    include_bytes!("../tablet2_p34.txt"),
    include_bytes!("../tablet2_p35.txt"),
    include_bytes!("../tablet2_p36.txt"),
    include_bytes!("../tablet2_p37.txt"),
    include_bytes!("../tablet2_p38.txt"),
    include_bytes!("../tablet2_p39.txt"),
    include_bytes!("../tablet2_p40.txt"),
    include_bytes!("../tablet2_p41.txt"),
    include_bytes!("../tablet2_p42.txt"),
    include_bytes!("../tablet2_p43.txt"),
];

static TABLET3: &[&[u8]] = &[
    include_bytes!("../tablet3_p44.txt"),
    include_bytes!("../tablet3_p45.txt"),
    include_bytes!("../tablet3_p46.txt"),
    include_bytes!("../tablet3_p47.txt"),
    include_bytes!("../tablet3_p48.txt"),
    include_bytes!("../tablet3_p49.txt"),
    include_bytes!("../tablet3_p50.txt"),
    include_bytes!("../tablet3_p51.txt"),
    include_bytes!("../tablet3_p52.txt"),
    include_bytes!("../tablet3_p53.txt"),
    include_bytes!("../tablet3_p54.txt"),
    include_bytes!("../tablet3_p55.txt"),
    include_bytes!("../tablet3_p56.txt"),
    include_bytes!("../tablet3_p57.txt"),
    include_bytes!("../tablet3_p58.txt"),
    include_bytes!("../tablet3_p59.txt"),
    include_bytes!("../tablet3_p60.txt"),
];

static TABLET4: &[&[u8]] = &[
    include_bytes!("../tablet4_p61.txt"),
    include_bytes!("../tablet4_p62.txt"),
    include_bytes!("../tablet4_p63.txt"),
    include_bytes!("../tablet4_p64.txt"),
    include_bytes!("../tablet4_p65.txt"),
    include_bytes!("../tablet4_p66.txt"),
    include_bytes!("../tablet4_p67.txt"),
    include_bytes!("../tablet4_p68.txt"),
    include_bytes!("../tablet4_p69.txt"),
    include_bytes!("../tablet4_p70.txt"),
    include_bytes!("../tablet4_p71.txt"),
    include_bytes!("../tablet4_p72.txt"),
    include_bytes!("../tablet4_p73.txt"),
    include_bytes!("../tablet4_p74.txt"),
    include_bytes!("../tablet4_p75.txt"),
    include_bytes!("../tablet4_p76.txt"),
    include_bytes!("../tablet4_p77.txt"),
];

static TABLET5: &[&[u8]] = &[
    include_bytes!("../tablet5_p78.txt"),
    include_bytes!("../tablet5_p79.txt"),
    include_bytes!("../tablet5_p80.txt"),
    include_bytes!("../tablet5_p81.txt"),
    include_bytes!("../tablet5_p82.txt"),
    include_bytes!("../tablet5_p83.txt"),
    include_bytes!("../tablet5_p84.txt"),
    include_bytes!("../tablet5_p85.txt"),
    include_bytes!("../tablet5_p86.txt"),
    include_bytes!("../tablet5_p87.txt"),
    include_bytes!("../tablet5_p88.txt"),
    include_bytes!("../tablet5_p89.txt"),
    include_bytes!("../tablet5_p90.txt"),
    include_bytes!("../tablet5_p91.txt"),
    include_bytes!("../tablet5_p92.txt"),
    include_bytes!("../tablet5_p93.txt"),
    include_bytes!("../tablet5_p94.txt"),
    include_bytes!("../tablet5_p95.txt"),
];

static TABLET6: &[&[u8]] = &[
    include_bytes!("../tablet6_p96.txt"),
    include_bytes!("../tablet6_p97.txt"),
    include_bytes!("../tablet6_p98.txt"),
    include_bytes!("../tablet6_p99.txt"),
    include_bytes!("../tablet6_p100.txt"),
    include_bytes!("../tablet6_p101.txt"),
    include_bytes!("../tablet6_p102.txt"),
    include_bytes!("../tablet6_p103.txt"),
    include_bytes!("../tablet6_p104.txt"),
    include_bytes!("../tablet6_p105.txt"),
    include_bytes!("../tablet6_p106.txt"),
    include_bytes!("../tablet6_p107.txt"),
    include_bytes!("../tablet6_p108.txt"),
    include_bytes!("../tablet6_p109.txt"),
    include_bytes!("../tablet6_p110.txt"),
    include_bytes!("../tablet6_p111.txt"),
    include_bytes!("../tablet6_p112.txt"),
];

static TABLET7: &[&[u8]] = &[
    include_bytes!("../tablet7_p113.txt"),
    include_bytes!("../tablet7_p114.txt"),
    include_bytes!("../tablet7_p115.txt"),
    include_bytes!("../tablet7_p116.txt"),
    include_bytes!("../tablet7_p117.txt"),
    include_bytes!("../tablet7_p118.txt"),
    include_bytes!("../tablet7_p119.txt"),
    include_bytes!("../tablet7_p120.txt"),
    include_bytes!("../tablet7_p121.txt"),
    include_bytes!("../tablet7_p122.txt"),
    include_bytes!("../tablet7_p123.txt"),
    include_bytes!("../tablet7_p124.txt"),
    include_bytes!("../tablet7_p125.txt"),
    include_bytes!("../tablet7_p126.txt"),
    include_bytes!("../tablet7_p127.txt"),
    include_bytes!("../tablet7_p128.txt"),
    include_bytes!("../tablet7_p129.txt"),
    include_bytes!("../tablet7_p130.txt"),
];

static TABLET8: &[&[u8]] = &[
    include_bytes!("../tablet8_p131.txt"),
    include_bytes!("../tablet8_p132.txt"),
    include_bytes!("../tablet8_p133.txt"),
    include_bytes!("../tablet8_p134.txt"),
    include_bytes!("../tablet8_p135.txt"),
    include_bytes!("../tablet8_p136.txt"),
    include_bytes!("../tablet8_p137.txt"),
    include_bytes!("../tablet8_p138.txt"),
    include_bytes!("../tablet8_p139.txt"),
    include_bytes!("../tablet8_p140.txt"),
    include_bytes!("../tablet8_p141.txt"),
    include_bytes!("../tablet8_p142.txt"),
    include_bytes!("../tablet8_p143.txt"),
    include_bytes!("../tablet8_p144.txt"),
    include_bytes!("../tablet8_p145.txt"),
    include_bytes!("../tablet8_p146.txt"),
    include_bytes!("../tablet8_p147.txt"),
];

static TABLET9: &[&[u8]] = &[
    include_bytes!("../tablet9_p148.txt"),
    include_bytes!("../tablet9_p149.txt"),
    include_bytes!("../tablet9_p150.txt"),
    include_bytes!("../tablet9_p151.txt"),
    include_bytes!("../tablet9_p152.txt"),
    include_bytes!("../tablet9_p153.txt"),
    include_bytes!("../tablet9_p154.txt"),
    include_bytes!("../tablet9_p155.txt"),
    include_bytes!("../tablet9_p156.txt"),
    include_bytes!("../tablet9_p157.txt"),
    include_bytes!("../tablet9_p158.txt"),
    include_bytes!("../tablet9_p159.txt"),
    include_bytes!("../tablet9_p160.txt"),
    include_bytes!("../tablet9_p161.txt"),
    include_bytes!("../tablet9_p162.txt"),
    include_bytes!("../tablet9_p163.txt"),
    include_bytes!("../tablet9_p164.txt"),
];

static TABLET10: &[&[u8]] = &[
    include_bytes!("../tablet10_p165.txt"),
    include_bytes!("../tablet10_p166.txt"),
    include_bytes!("../tablet10_p167.txt"),
    include_bytes!("../tablet10_p168.txt"),
    include_bytes!("../tablet10_p169.txt"),
    include_bytes!("../tablet10_p170.txt"),
    include_bytes!("../tablet10_p171.txt"),
    include_bytes!("../tablet10_p172.txt"),
    include_bytes!("../tablet10_p173.txt"),
    include_bytes!("../tablet10_p174.txt"),
    include_bytes!("../tablet10_p175.txt"),
    include_bytes!("../tablet10_p176.txt"),
    include_bytes!("../tablet10_p177.txt"),
    include_bytes!("../tablet10_p178.txt"),
    include_bytes!("../tablet10_p179.txt"),
    include_bytes!("../tablet10_p180.txt"),
    include_bytes!("../tablet10_p181.txt"),
    include_bytes!("../tablet10_p182.txt"),
];

static TABLET11: &[&[u8]] = &[
    include_bytes!("../tablet11_p183.txt"),
    include_bytes!("../tablet11_p184.txt"),
    include_bytes!("../tablet11_p185.txt"),
    include_bytes!("../tablet11_p186.txt"),
    include_bytes!("../tablet11_p187.txt"),
    include_bytes!("../tablet11_p188.txt"),
    include_bytes!("../tablet11_p189.txt"),
    include_bytes!("../tablet11_p190.txt"),
    include_bytes!("../tablet11_p191.txt"),
    include_bytes!("../tablet11_p192.txt"),
    include_bytes!("../tablet11_p193.txt"),
    include_bytes!("../tablet11_p194.txt"),
    include_bytes!("../tablet11_p195.txt"),
    include_bytes!("../tablet11_p196.txt"),
    include_bytes!("../tablet11_p197.txt"),
    include_bytes!("../tablet11_p198.txt"),
    include_bytes!("../tablet11_p199.txt"),
    include_bytes!("../tablet11_p200.txt"),
];

static TABLET12: &[&[u8]] = &[
    include_bytes!("../tablet12_p201.txt"),
    include_bytes!("../tablet12_p202.txt"),
    include_bytes!("../tablet12_p203.txt"),
    include_bytes!("../tablet12_p204.txt"),
    include_bytes!("../tablet12_p205.txt"),
    include_bytes!("../tablet12_p206.txt"),
    include_bytes!("../tablet12_p207.txt"),
    include_bytes!("../tablet12_p208.txt"),
    include_bytes!("../tablet12_p209.txt"),
    include_bytes!("../tablet12_p210.txt"),
    include_bytes!("../tablet12_p211.txt"),
    include_bytes!("../tablet12_p212.txt"),
    include_bytes!("../tablet12_p213.txt"),
    include_bytes!("../tablet12_p214.txt"),
    include_bytes!("../tablet12_p215.txt"),
    include_bytes!("../tablet12_p216.txt"),
    include_bytes!("../tablet12_p217.txt"),
];

static TABLET13: &[&[u8]] = &[
    include_bytes!("../tablet13_p218.txt"),
    include_bytes!("../tablet13_p219.txt"),
    include_bytes!("../tablet13_p220.txt"),
    include_bytes!("../tablet13_p221.txt"),
    include_bytes!("../tablet13_p222.txt"),
    include_bytes!("../tablet13_p223.txt"),
    include_bytes!("../tablet13_p224.txt"),
    include_bytes!("../tablet13_p225.txt"),
    include_bytes!("../tablet13_p226.txt"),
    include_bytes!("../tablet13_p227.txt"),
    include_bytes!("../tablet13_p228.txt"),
    include_bytes!("../tablet13_p229.txt"),
    include_bytes!("../tablet13_p230.txt"),
    include_bytes!("../tablet13_p231.txt"),
    include_bytes!("../tablet13_p232.txt"),
    include_bytes!("../tablet13_p233.txt"),
];

static TABLET14: &[&[u8]] = &[
    include_bytes!("../tablet14_p234.txt"),
    include_bytes!("../tablet14_p235.txt"),
    include_bytes!("../tablet14_p236.txt"),
    include_bytes!("../tablet14_p237.txt"),
];

static GLOSSARY: &[&[u8]] = &[
    include_bytes!("../glossary_p238.txt"),
    include_bytes!("../glossary_p239.txt"),
    include_bytes!("../glossary_p240.txt"),
    include_bytes!("../glossary_p241.txt"),
    include_bytes!("../glossary_p242.txt"),
    include_bytes!("../glossary_p243.txt"),
    include_bytes!("../glossary_p244.txt"),
    include_bytes!("../glossary_p245.txt"),
    include_bytes!("../glossary_p246.txt"),
];

// ── Chapter catalogue ──────────────────────────────────────────────────────────

struct Chapter {
    title: &'static str,
    pages: &'static [&'static [u8]],
}

static CHAPTERS: &[Chapter] = &[
    Chapter { title: "Introduction",  pages: INTRO    },
    Chapter { title: "Tablet  I",     pages: TABLET1  },
    Chapter { title: "Tablet  II",    pages: TABLET2  },
    Chapter { title: "Tablet  III",   pages: TABLET3  },
    Chapter { title: "Tablet  IV",    pages: TABLET4  },
    Chapter { title: "Tablet  V",     pages: TABLET5  },
    Chapter { title: "Tablet  VI",    pages: TABLET6  },
    Chapter { title: "Tablet  VII",   pages: TABLET7  },
    Chapter { title: "Tablet  VIII",  pages: TABLET8  },
    Chapter { title: "Tablet  IX",    pages: TABLET9  },
    Chapter { title: "Tablet  X",     pages: TABLET10 },
    Chapter { title: "Tablet  XI",    pages: TABLET11 },
    Chapter { title: "Tablet  XII",   pages: TABLET12 },
    Chapter { title: "Tablet  XIII",  pages: TABLET13 },
    Chapter { title: "Tablet  XIV",   pages: TABLET14 },
    Chapter { title: "Glossary",      pages: GLOSSARY },
];

// ── App state ──────────────────────────────────────────────────────────────────

#[derive(PartialEq)]
enum View {
    Toc,
    Reading,
}

struct App {
    view: View,
    toc_state: ListState,
    // Reading state
    chapter_idx: usize,
    page_idx: usize,
    scroll: usize,
    // Cached wrapped lines for current page
    wrapped_lines: Vec<String>,
    // Scrollbar state
    scrollbar_state: ScrollbarState,
}

impl App {
    fn new() -> Self {
        let mut toc_state = ListState::default();
        toc_state.select(Some(0));
        App {
            view: View::Toc,
            toc_state,
            chapter_idx: 0,
            page_idx: 0,
            scroll: 0,
            wrapped_lines: Vec::new(),
            scrollbar_state: ScrollbarState::default(),
        }
    }

    fn open_chapter(&mut self, chapter_idx: usize, content_width: u16) {
        self.chapter_idx = chapter_idx;
        self.page_idx = 0;
        self.scroll = 0;
        self.view = View::Reading;
        self.rebuild_cache(content_width);
    }

    fn rebuild_cache(&mut self, width: u16) {
        let raw = CHAPTERS[self.chapter_idx].pages[self.page_idx];
        let text = String::from_utf8_lossy(raw);
        self.wrapped_lines = wrap_text(&text, width as usize);
        let total = self.wrapped_lines.len().saturating_sub(1);
        self.scrollbar_state = ScrollbarState::new(total).position(self.scroll);
    }

    fn scroll_down(&mut self, content_height: usize) {
        let max = self.wrapped_lines.len().saturating_sub(content_height);
        if self.scroll < max {
            self.scroll += 1;
            self.scrollbar_state = self.scrollbar_state.position(self.scroll);
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
            self.scrollbar_state = self.scrollbar_state.position(self.scroll);
        }
    }

    fn next_page(&mut self, content_width: u16) {
        let total_pages = CHAPTERS[self.chapter_idx].pages.len();
        if self.page_idx + 1 < total_pages {
            self.page_idx += 1;
            self.scroll = 0;
            self.rebuild_cache(content_width);
        }
    }

    fn prev_page(&mut self, content_width: u16) {
        if self.page_idx > 0 {
            self.page_idx -= 1;
            self.scroll = 0;
            self.rebuild_cache(content_width);
        }
    }

    fn toc_next(&mut self) {
        let i = self.toc_state.selected().unwrap_or(0);
        self.toc_state
            .select(Some((i + 1).min(CHAPTERS.len() - 1)));
    }

    fn toc_prev(&mut self) {
        let i = self.toc_state.selected().unwrap_or(0);
        self.toc_state.select(Some(i.saturating_sub(1)));
    }
}

// ── Text wrapping ──────────────────────────────────────────────────────────────

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![];
    }
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        // Replace control chars / non-printable sequences
        let para = paragraph.replace('\r', "").replace('\u{FFFC}', "");
        if para.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        // Simple word-wrap
        let mut current = String::new();
        let mut col = 0usize;
        for word in para.split_whitespace() {
            let wlen = word.chars().count();
            if col == 0 {
                current.push_str(word);
                col = wlen;
            } else if col + 1 + wlen <= width {
                current.push(' ');
                current.push_str(word);
                col += 1 + wlen;
            } else {
                lines.push(std::mem::take(&mut current));
                current.push_str(word);
                col = wlen;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
}

// ── Rendering ─────────────────────────────────────────────────────────────────

const AMBER:  Color = Color::Rgb(255, 176, 0);
const GOLD:   Color = Color::Rgb(212, 175, 55);
const STONE:  Color = Color::Rgb(200, 185, 150);
const DIM:    Color = Color::Rgb(120, 110, 80);
const BG:     Color = Color::Rgb(18, 14, 8);

fn render_toc(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Outer block
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(GOLD))
        .title(Line::from(vec![
            Span::styled(" ✦ ", Style::default().fg(AMBER)),
            Span::styled(
                "THE LOST BOOK OF ENKI",
                Style::default()
                    .fg(AMBER)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ✦ ", Style::default().fg(AMBER)),
        ]))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(BG));

    let inner = outer.inner(area);
    f.render_widget(outer, area);

    // Layout: subtitle + list + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(inner);

    // Subtitle
    let subtitle = Paragraph::new(Line::from(vec![Span::styled(
        "Memoirs and Prophecies of an Extraterrestrial God",
        Style::default().fg(STONE).add_modifier(Modifier::ITALIC),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(subtitle, chunks[0]);

    // Chapter list
    let items: Vec<ListItem> = CHAPTERS
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let pages = ch.pages.len();
            let line = Line::from(vec![
                Span::styled(
                    format!(" {:>2}. ", i + 1),
                    Style::default().fg(DIM),
                ),
                Span::styled(
                    ch.title,
                    Style::default().fg(STONE).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  ({} page{})", pages, if pages == 1 { "" } else { "s" }),
                    Style::default().fg(DIM),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(DIM))
                .title(Span::styled(
                    " Table of Contents ",
                    Style::default().fg(GOLD),
                )),
        )
        .highlight_style(
            Style::default()
                .fg(AMBER)
                .bg(Color::Rgb(40, 30, 10))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut app.toc_state);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(AMBER)),
        Span::styled("Navigate  ", Style::default().fg(DIM)),
        Span::styled(" Enter ", Style::default().fg(AMBER)),
        Span::styled("Open  ", Style::default().fg(DIM)),
        Span::styled(" q ", Style::default().fg(AMBER)),
        Span::styled("Quit", Style::default().fg(DIM)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn render_reading(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let ch = &CHAPTERS[app.chapter_idx];
    let total_pages = ch.pages.len();

    // Outer block
    let title_text = format!(
        " {} — page {}/{} ",
        ch.title,
        app.page_idx + 1,
        total_pages
    );
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(GOLD))
        .title(Line::from(vec![
            Span::styled(" ✦ ", Style::default().fg(AMBER)),
            Span::styled(
                title_text,
                Style::default().fg(AMBER).add_modifier(Modifier::BOLD),
            ),
            Span::styled("✦ ", Style::default().fg(AMBER)),
        ]))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(BG));

    let inner = outer.inner(area);
    f.render_widget(outer, area);

    // Layout: content + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(inner);

    let content_area = chunks[0];
    let content_height = content_area.height as usize;
    let content_width = content_area.width.saturating_sub(2); // leave room for scrollbar

    // Rebuild cache if width changed (resize)
    if app.wrapped_lines.is_empty() {
        app.rebuild_cache(content_width);
    }

    // Slice visible lines
    let visible: Vec<Line> = app
        .wrapped_lines
        .iter()
        .skip(app.scroll)
        .take(content_height)
        .map(|l| {
            if l.is_empty() {
                Line::default()
            } else {
                Line::from(Span::styled(l.clone(), Style::default().fg(STONE)))
            }
        })
        .collect();

    let para = Paragraph::new(visible)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(DIM)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(para, content_area);

    // Scrollbar
    let total_lines = app.wrapped_lines.len();
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .track_symbol(Some("│"))
        .thumb_symbol("█");

    app.scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(content_height))
        .position(app.scroll);

    f.render_stateful_widget(
        scrollbar,
        content_area.inner(Margin { vertical: 1, horizontal: 0 }),
        &mut app.scrollbar_state,
    );

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(AMBER)),
        Span::styled("Scroll  ", Style::default().fg(DIM)),
        Span::styled(" ←→ ", Style::default().fg(AMBER)),
        Span::styled("Prev/Next Page  ", Style::default().fg(DIM)),
        Span::styled(" Esc ", Style::default().fg(AMBER)),
        Span::styled("Contents  ", Style::default().fg(DIM)),
        Span::styled(" q ", Style::default().fg(AMBER)),
        Span::styled("Quit", Style::default().fg(DIM)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[1]);
}

// ── Main ───────────────────────────────────────────────────────────────────────

fn main() -> Result<(), io::Error> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut last_width: u16 = 0;

    loop {
        terminal.draw(|f| {
            match app.view {
                View::Toc => render_toc(f, &mut app),
                View::Reading => render_reading(f, &mut app),
            }
        })?;

        // Track terminal width for reflow
        let current_width = terminal.size()?.width;
        if app.view == View::Reading && current_width != last_width {
            let w = current_width.saturating_sub(4); // borders + scrollbar
            app.rebuild_cache(w);
            last_width = current_width;
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app.view {
                    View::Toc => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Down | KeyCode::Char('j') => app.toc_next(),
                        KeyCode::Up | KeyCode::Char('k') => app.toc_prev(),
                        KeyCode::Enter | KeyCode::Char('l') => {
                            if let Some(idx) = app.toc_state.selected() {
                                let w = terminal
                                    .size()
                                    .map(|s| s.width.saturating_sub(4))
                                    .unwrap_or(80);
                                app.open_chapter(idx, w);
                                last_width = terminal.size()?.width;
                            }
                        }
                        _ => {}
                    },
                    View::Reading => {
                        let size = terminal.size()?;
                        let content_height = size.height.saturating_sub(6) as usize;
                        let content_width = size.width.saturating_sub(4);
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            KeyCode::Esc | KeyCode::Char('h') => {
                                app.view = View::Toc;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.scroll_down(content_height);
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.scroll_up();
                            }
                            KeyCode::Right | KeyCode::Char('n') | KeyCode::PageDown => {
                                app.next_page(content_width);
                            }
                            KeyCode::Left | KeyCode::Char('p') | KeyCode::PageUp => {
                                app.prev_page(content_width);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
