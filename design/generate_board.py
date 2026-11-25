"""Generates winisland-board.svg (import into Figma). Run: python generate_board.py"""
FONT = "Segoe UI Variable, Segoe UI, Inter, sans-serif"
ACCENT = "#4CA0FF"; GREEN = "#34D17A"; ORANGE = "#FF9F0A"; PURPLE = "#B07CFF"; TEAL = "#3AD6C4"
out = []
add = out.append

def text(x, y, s, size=13, weight=400, fill="#FFFFFF", anchor="start", op=1.0):
    add(f'<text x="{x}" y="{y}" font-family="{FONT}" font-size="{size}" font-weight="{weight}" '
        f'fill="{fill}" fill-opacity="{op}" text-anchor="{anchor}">{s}</text>')

def glass(w, h, r, base="#0A0E18", base_op=0.62, border=True, stroke_override=None, hl=True):
    add(f'<rect width="{w}" height="{h}" rx="{r}" fill="{base}" fill-opacity="{base_op}" filter="url(#shadow)"/>')
    if hl:
        add(f'<rect width="{w}" height="{h}" rx="{r}" fill="url(#hl)"/>')
    if border:
        st = stroke_override or "url(#stroke)"
        add(f'<rect x="0.5" y="0.5" width="{w-1}" height="{h-1}" rx="{r-0.5}" fill="none" stroke="{st}" stroke-width="1"/>')

def group(name, x, y):
    add(f'<g id="{name}" transform="translate({x} {y})">')
def end(): add('</g>')

def dots(cx, cy, color="#FFFFFF"):
    add(f'<circle cx="{cx-6}" cy="{cy}" r="3" fill="{color}" fill-opacity="0.92"/>')
    add(f'<circle cx="{cx+6}" cy="{cy}" r="3" fill="{color}" fill-opacity="0.92"/>')

def more(cx, cy, color="#FFFFFF"):
    for dx in (-8, 0, 8):
        add(f'<circle cx="{cx+dx}" cy="{cy}" r="1.9" fill="{color}" fill-opacity="0.85"/>')

def icon(name, cx, cy, color="#FFFFFF", op=0.9):
    s = f'fill="none" stroke="{color}" stroke-opacity="{op}" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"'
    add(f'<g id="icon/{name}" {s}>')
    if name == "plus": add(f'<path d="M{cx-7} {cy}H{cx+7}M{cx} {cy-7}V{cy+7}"/>')
    elif name == "home": add(f'<path d="M{cx-8} {cy+1}L{cx} {cy-7}L{cx+8} {cy+1}M{cx-5} {cy-2}V{cy+7}H{cx+5}V{cy-2}"/>')
    elif name == "grid":
        for dx in (-7, 1):
            for dy in (-7, 1): add(f'<rect x="{cx+dx}" y="{cy+dy}" width="6" height="6" rx="1.5"/>')
    elif name == "music": add(f'<circle cx="{cx-3}" cy="{cy+5}" r="2.6"/><path d="M{cx-0.4} {cy+5}V{cy-7}L{cx+6} {cy-5}"/>')
    elif name == "image": add(f'<rect x="{cx-8}" y="{cy-6}" width="16" height="12" rx="2.5"/><circle cx="{cx-3}" cy="{cy-2}" r="1.5"/><path d="M{cx-8} {cy+4}L{cx-2} {cy}L{cx+8} {cy+5}"/>')
    elif name == "moon": add(f'<path d="M{cx+5} {cy+5}A7 7 0 1 1 {cx+1} {cy-7}A5.5 5.5 0 0 0 {cx+5} {cy+5}Z"/>')
    elif name == "calendar": add(f'<rect x="{cx-7}" y="{cy-6}" width="14" height="13" rx="2.5"/><path d="M{cx-7} {cy-2}H{cx+7}M{cx-3} {cy-8}V{cy-4}M{cx+3} {cy-8}V{cy-4}"/>')
    elif name == "mail": add(f'<rect x="{cx-8}" y="{cy-5}" width="16" height="11" rx="2.5"/><path d="M{cx-8} {cy-4}L{cx} {cy+2}L{cx+8} {cy-4}"/>')
    elif name == "gear": add(f'<circle cx="{cx}" cy="{cy}" r="7"/><circle cx="{cx}" cy="{cy}" r="2.6"/>')
    elif name == "bell": add(f'<path d="M{cx-5} {cy+3}V{cy-1}A5 5 0 0 1 {cx+5} {cy-1}V{cy+3}L{cx+6.5} {cy+5}H{cx-6.5}Z"/><path d="M{cx-1.5} {cy+7.5}H{cx+1.5}"/>')
    elif name == "check": add(f'<rect x="{cx-8}" y="{cy-8}" width="16" height="16" rx="4"/><path d="M{cx-4} {cy}L{cx-1} {cy+3}L{cx+4} {cy-3}"/>')
    elif name == "prev": add(f'<path d="M{cx+5} {cy-5}L{cx-2} {cy}L{cx+5} {cy+5}Z M{cx-5} {cy-5}V{cy+5}"/>')
    elif name == "next": add(f'<path d="M{cx-5} {cy-5}L{cx+2} {cy}L{cx-5} {cy+5}Z M{cx+5} {cy-5}V{cy+5}"/>')
    add('</g>')

def bolt(cx, cy, color=GREEN):
    add(f'<path id="icon/bolt" d="M{cx+1.5} {cy-7}L{cx-4} {cy+1}H{cx-0.5}L{cx-1.5} {cy+7}L{cx+4} {cy-1}H{cx+0.5}Z" fill="{color}"/>')

def play_btn(cx, cy, r=12):
    add(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="#FFFFFF"/>')
    add(f'<path d="M{cx-3} {cy-5}L{cx+5} {cy}L{cx-3} {cy+5}Z" fill="#0A0E18"/>')

def pause_btn(cx, cy, r=12):
    add(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="#FFFFFF"/>')
    add(f'<rect x="{cx-4}" y="{cy-5}" width="3" height="10" rx="1" fill="#0A0E18"/><rect x="{cx+1}" y="{cy-5}" width="3" height="10" rx="1" fill="#0A0E18"/>')

def art(x, y, s, r):
    add(f'<rect x="{x}" y="{y}" width="{s}" height="{s}" rx="{r}" fill="url(#art)"/>')

def ring(cx, cy, pct, color, r=14):
    import math
    c = 2 * math.pi * r
    add(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="none" stroke="#FFFFFF" stroke-opacity="0.16" stroke-width="3.5"/>')
    add(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="none" stroke="{color}" stroke-width="3.5" stroke-linecap="round" '
        f'stroke-dasharray="{c*pct/100:.1f} {c:.1f}" transform="rotate(-90 {cx} {cy})"/>')

def wave(x, cy):
    for i, h in enumerate([8, 14, 10, 17, 9]):
        add(f'<rect x="{x+i*6}" y="{cy-h/2}" width="3" height="{h}" rx="1.5" fill="{ACCENT}"/>')

def label(cx, y, name, desc):
    text(cx, y, name, 13, 600, anchor="middle")
    text(cx, y + 17, desc, 11.5, 400, anchor="middle", op=0.6)

W, H = 1280, 980
add(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">')
add('''<defs>
<linearGradient id="wall" x1="0" y1="0" x2="0.4" y2="1"><stop offset="0" stop-color="#0B1F57"/><stop offset="0.55" stop-color="#1747A8"/><stop offset="1" stop-color="#2B78E6"/></linearGradient>
<linearGradient id="hl" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#FFFFFF" stop-opacity="0.12"/><stop offset="1" stop-color="#FFFFFF" stop-opacity="0.01"/></linearGradient>
<linearGradient id="stroke" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#FFFFFF" stop-opacity="0.38"/><stop offset="1" stop-color="#FFFFFF" stop-opacity="0.07"/></linearGradient>
<linearGradient id="art" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#7A5CFF"/><stop offset="0.6" stop-color="#E86AA8"/><stop offset="1" stop-color="#FF9A5A"/></linearGradient>
<filter id="shadow" x="-20%" y="-30%" width="140%" height="180%"><feDropShadow dx="0" dy="8" stdDeviation="12" flood-color="#000000" flood-opacity="0.35"/></filter>
</defs>''')
add(f'<rect id="Wallpaper" width="{W}" height="{H}" fill="url(#wall)"/>')
add('<ellipse cx="980" cy="260" rx="420" ry="240" fill="#3C8CFF" fill-opacity="0.16"/><ellipse cx="300" cy="900" rx="520" ry="200" fill="#0A2A7A" fill-opacity="0.35"/>')
text(60, 56, "WinIsland - Design board", 26, 700)
text(60, 78, "Fluent / Liquid Glass. Apply Background blur 20-30 to glass layers in Figma.", 13, 400, op=0.65)

# ---------- Expanded Home ----------
text(60, 112, "Expanded - Home", 18, 600)
group("Island/Expanded-Home", 60, 128)
glass(460, 250, 36)
text(24, 38, "Wed, 30 Sept / 28°", 13, 600)
dots(230, 34)
more(436, 34)
text(230, 108, "10:24", 64, 700, anchor="middle")
for i, cx in enumerate((20, 163, 306)):
    add(f'<rect id="card/{i+1}" x="{cx}" y="128" width="134" height="54" rx="18" fill="#FFFFFF" fill-opacity="0.09"/>')
icon("check", 40, 155)
text(62, 153, "3", 16, 700); text(62, 170, "tasks today", 11.5, 400, op=0.65)
art(172, 140, 30, 8)
text(208, 154, "After Hours", 11, 600); text(208, 169, "The Weeknd", 10.5, 400, op=0.65)
pause_btn(280, 155, 10)
text(322, 154, "25:00", 16, 700); text(322, 170, "Focus, ready", 11.5, 400, op=0.65)
play_btn(421, 155, 11)
icon("plus", 34, 222, op=0.85)
add('<rect id="nav/active" x="62" y="204" width="86" height="36" rx="18" fill="#FFFFFF" fill-opacity="0.14"/>')
icon("home", 82, 222); text(96, 227, "Home", 13, 600)
for n, cx in (("grid", 178), ("music", 216), ("image", 254), ("moon", 292), ("calendar", 330), ("mail", 368)):
    icon(n, cx, 222, op=0.8)
icon("gear", 430, 222, op=0.8)
end()

# ---------- Island states ----------
text(60, 430, "Island States", 18, 600)
y = 450
group("Island/Idle", 60, y); glass(126, 34, 17, base_op=0.85); dots(63, 17); end()
label(123, y + 56, "Idle", "Minimal and clean")
group("Island/Media", 240, y); glass(200, 34, 17, base_op=0.85); art(8, 5, 24, 6); wave(150, 17); end()
label(340, y + 56, "Media", "While playing media")
group("Island/Charging", 500, y); glass(150, 34, 17, base_op=0.85)
add(f'<circle cx="22" cy="17" r="11" fill="none" stroke="{GREEN}" stroke-opacity="0.9" stroke-width="2"/>'); bolt(22, 17)
text(46, 22, "72%", 13, 600); end()
label(575, y + 56, "Charging", "With battery status")
group("Island/Notification", 720, y); glass(120, 34, 17, base_op=0.85); icon("bell", 52, 17, op=0.95)
add(f'<circle cx="64" cy="10" r="3.5" fill="{ORANGE}"/>'); end()
label(780, y + 56, "Notification", "For new alerts")
group("Island/Focus", 900, y); glass(150, 34, 17, base_op=0.85); text(18, 22, "25:00", 14, 600); play_btn(128, 17, 10); end()
label(975, y + 56, "Focus", "Active session")

# ---------- Expanded states ----------
text(60, 600, "Expanded States", 18, 600)
y = 620
group("Island/Expanded-Media", 60, y); glass(360, 130, 28)
art(16, 16, 52, 12); text(80, 36, "After Hours", 14, 600); text(80, 54, "The Weeknd", 12, 400, op=0.65); more(334, 28)
text(80, 90, "1:24", 11, 400, op=0.65)
add('<rect x="108" y="84" width="188" height="4" rx="2" fill="#FFFFFF" fill-opacity="0.2"/><rect x="108" y="84" width="62" height="4" rx="2" fill="#FFFFFF"/><circle cx="170" cy="86" r="5" fill="#FFFFFF"/>')
text(304, 90, "3:58", 11, 400, op=0.65)
icon("prev", 150, 112); pause_btn(180, 112, 11); icon("next", 210, 112); end()
group("Island/Expanded-Performance", 440, y); glass(340, 130, 28)
for cx, nm, v, col in ((50, "CPU", 12, ACCENT), (160, "GPU", 38, PURPLE), (270, "RAM", 61, TEAL)):
    ring(cx, 65, v, col); text(cx + 24, 62, nm, 11, 400, op=0.65); text(cx + 24, 78, f"{v}%", 13, 600)
end()
group("Island/Expanded-Notification", 800, y); glass(380, 90, 28)
add('<rect x="18" y="23" width="44" height="44" rx="11" fill="#1B6FE0"/>'); text(40, 53, "O", 22, 700, anchor="middle")
text(76, 40, "New message", 14, 600); text(362, 40, "2 min ago", 12, 400, anchor="end", op=0.65)
text(76, 62, "Project update from Robotics Club", 12.5, 400, op=0.8); end()

# ---------- Style variants ----------
text(60, 800, "Style Variants", 18, 600)
y = 825
variants = [
    ("Default (Fluent)", "Windows 11 native look", dict(base_op=0.62, stroke_override=ACCENT), "#FFFFFF"),
    ("Light", "Clean and minimal", dict(base="#E6ECF6", base_op=0.88), "#141820"),
    ("Acrylic", "Blurred transparency", dict(base="#2A2E38", base_op=0.55), "#FFFFFF"),
    ("Mica", "Wallpaper adaptive", dict(base="#1E2430", base_op=0.80), "#FFFFFF"),
    ("Compact", "Minimal footprint", None, "#FFFFFF"),
    ("Glass", "Translucent and airy", dict(base="#FFFFFF", base_op=0.12, stroke_override="#FFFFFF"), "#FFFFFF"),
]
for i, (nm, ds, kw, tc) in enumerate(variants):
    x = 60 + i * 196
    group(f"Variant/{nm}", x, y)
    if kw is None:
        glass(160, 34, 17, base_op=0.9); more(24, 17); dots(80, 17); more(136, 17)
        end(); label(x + 80, y + 66, nm, ds)
        continue
    glass(160, 64, 22, **kw)
    dots(80, 16, tc); more(142, 16, tc); text(80, 50, "10:24", 22, 700, fill=tc, anchor="middle")
    end(); label(x + 80, y + 96, nm, ds)

add('</svg>')
open("winisland-board.svg", "w", encoding="utf-8").write("\n".join(out))
print("ok", len("\n".join(out)))
