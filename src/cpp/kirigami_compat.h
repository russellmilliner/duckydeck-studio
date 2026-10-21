#pragma once

#include <QApplication>
#include <QColor>
#include <QFont>
#include <QGuiApplication>
#include <QObject>
#include <QQuickItem>
#include <QQuickTextDocument>
#include <QRegularExpression>
#include <QSyntaxHighlighter>
#include <QTextCharFormat>
#include <QUrl>
#include <QVariant>
#include <QVector>
#include <qqml.h>

namespace duckydeck {

class KirigamiTheme : public QObject {
    Q_OBJECT
    QML_NAMED_ELEMENT(Theme)
    QML_ATTACHED(duckydeck::KirigamiTheme)
    QML_UNCREATABLE("Attached Property")

    Q_PROPERTY(ColorSet colorSet READ colorSet WRITE setColorSet NOTIFY colorsChanged FINAL)
    Q_PROPERTY(bool inherit READ inherit WRITE setInherit NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor backgroundColor READ backgroundColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor alternateBackgroundColor READ alternateBackgroundColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor textColor READ textColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor disabledTextColor READ disabledTextColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor highlightColor READ highlightColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor highlightedTextColor READ highlightedTextColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor positiveTextColor READ positiveTextColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor negativeTextColor READ negativeTextColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor neutralTextColor READ neutralTextColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor neutralBackgroundColor READ neutralBackgroundColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor focusColor READ focusColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QColor separatorColor READ separatorColor NOTIFY colorsChanged FINAL)
    Q_PROPERTY(double frameContrast READ frameContrast NOTIFY colorsChanged FINAL)
    Q_PROPERTY(QFont defaultFont READ defaultFont NOTIFY colorsChanged FINAL)

public:
    enum ColorSet {
        Window = 0,
        View = 1,
        Button = 2,
        Selection = 3,
        Tooltip = 4,
        Complementary = 5,
        Header = 6
    };
    Q_ENUM(ColorSet)

    explicit KirigamiTheme(QObject* parent = nullptr) : QObject(parent) {
        instances().append(this);
    }

    ~KirigamiTheme() override {
        instances().removeAll(this);
    }

    static KirigamiTheme* qmlAttachedProperties(QObject* object) {
        return new KirigamiTheme(object);
    }

    static bool& isLightModeRef() {
        static bool s_light = false;
        return s_light;
    }

    static QVector<KirigamiTheme*>& instances() {
        static QVector<KirigamiTheme*> s_list;
        return s_list;
    }

    static void setLightMode(bool light) {
        if (isLightModeRef() == light) {
            return;
        }
        isLightModeRef() = light;
        const auto copy = instances();
        for (KirigamiTheme* inst : copy) {
            if (inst) {
                Q_EMIT inst->colorsChanged();
            }
        }
    }

    ColorSet colorSet() const { return m_colorSet; }
    void setColorSet(ColorSet cs) {
        if (m_colorSet != cs || !m_explicitColorSet) {
            m_colorSet = cs;
            m_explicitColorSet = true;
            Q_EMIT colorsChanged();
        }
    }

    bool inherit() const { return m_inherit; }
    void setInherit(bool inh) {
        if (m_inherit != inh) {
            m_inherit = inh;
            Q_EMIT colorsChanged();
        }
    }

    ColorSet effectiveColorSet() const {
        if (!m_inherit || m_explicitColorSet) {
            return m_colorSet;
        }
        QObject* cur = parent();
        while (cur) {
            if (auto* item = qobject_cast<QQuickItem*>(cur)) {
                cur = item->parentItem() ? static_cast<QObject*>(item->parentItem()) : cur->parent();
            } else {
                cur = cur->parent();
            }
            if (!cur) break;
            if (auto* attached = qobject_cast<KirigamiTheme*>(
                    qmlAttachedPropertiesObject<KirigamiTheme>(cur, false))) {
                if (!attached->inherit() || attached->m_explicitColorSet) {
                    return attached->colorSet();
                }
            }
        }
        return m_colorSet;
    }

    QColor backgroundColor() const {
        const bool light = isLightModeRef();
        const ColorSet cs = effectiveColorSet();
        if (light) {
            return (cs == View) ? QColor(0xff, 0xff, 0xff) : QColor(0xef, 0xf0, 0xf1);
        }
        return (cs == View) ? QColor(0x1b, 0x1e, 0x20) : QColor(0x20, 0x23, 0x26);
    }

    QColor alternateBackgroundColor() const {
        const bool light = isLightModeRef();
        const ColorSet cs = effectiveColorSet();
        if (light) {
            return (cs == View) ? QColor(0xf7, 0xf7, 0xf7) : QColor(0xe3, 0xe5, 0xe7);
        }
        return (cs == View) ? QColor(0x23, 0x26, 0x29) : QColor(0x29, 0x2c, 0x30);
    }

    QColor textColor() const {
        return isLightModeRef() ? QColor(0x23, 0x26, 0x29) : QColor(0xfc, 0xfc, 0xfc);
    }

    QColor disabledTextColor() const {
        return isLightModeRef() ? QColor(0x7f, 0x8c, 0x8d) : QColor(0xa1, 0xa9, 0xb1);
    }

    QColor highlightColor() const {
        return QColor(0x3d, 0xae, 0xe9);
    }

    QColor highlightedTextColor() const {
        return QColor(0xff, 0xff, 0xff);
    }

    QColor positiveTextColor() const {
        return isLightModeRef() ? QColor(0x1d, 0x99, 0x53) : QColor(0x27, 0xae, 0x60);
    }

    QColor negativeTextColor() const {
        return isLightModeRef() ? QColor(0xda, 0x44, 0x53) : QColor(0xf4, 0x4f, 0x5d);
    }

    QColor neutralTextColor() const {
        return isLightModeRef() ? QColor(0xc9, 0x67, 0x00) : QColor(0xf6, 0x74, 0x00);
    }

    QColor neutralBackgroundColor() const {
        return isLightModeRef() ? QColor(0xff, 0xf6, 0xe5) : QColor(0x33, 0x29, 0x1a);
    }

    QColor focusColor() const {
        return QColor(0x3d, 0xae, 0xe9);
    }

    QColor separatorColor() const {
        return isLightModeRef() ? QColor(0xd5, 0xd8, 0xdc) : QColor(0x3b, 0x40, 0x45);
    }

    double frameContrast() const { return 0.20; }

    QFont defaultFont() const { return QGuiApplication::font(); }

Q_SIGNALS:
    void colorsChanged();

private:
    ColorSet m_colorSet = Window;
    bool m_inherit = true;
    bool m_explicitColorSet = false;
};

class KirigamiUnits : public QObject {
    Q_OBJECT
    Q_PROPERTY(int smallSpacing READ smallSpacing CONSTANT FINAL)
    Q_PROPERTY(int mediumSpacing READ mediumSpacing CONSTANT FINAL)
    Q_PROPERTY(int largeSpacing READ largeSpacing CONSTANT FINAL)
    Q_PROPERTY(int gridUnit READ gridUnit CONSTANT FINAL)

public:
    explicit KirigamiUnits(QObject* parent = nullptr) : QObject(parent) {}
    int smallSpacing() const { return 4; }
    int mediumSpacing() const { return 6; }
    int largeSpacing() const { return 8; }
    int gridUnit() const { return 18; }
};

class KirigamiGlobalToolBar : public QObject {
    Q_OBJECT
    Q_PROPERTY(int style READ style WRITE setStyle NOTIFY styleChanged FINAL)
public:
    explicit KirigamiGlobalToolBar(QObject* parent = nullptr) : QObject(parent) {}
    int style() const { return m_style; }
    void setStyle(int s) {
        if (m_style != s) {
            m_style = s;
            Q_EMIT styleChanged();
        }
    }
Q_SIGNALS:
    void styleChanged();
private:
    int m_style = 0;
};

class KirigamiPageStack : public QObject {
    Q_OBJECT
    Q_PROPERTY(duckydeck::KirigamiGlobalToolBar* globalToolBar READ globalToolBar CONSTANT FINAL)
    Q_PROPERTY(QQuickItem* initialPage READ initialPage WRITE setInitialPage NOTIFY initialPageChanged FINAL)
public:
    explicit KirigamiPageStack(QObject* parent = nullptr)
        : QObject(parent), m_globalToolBar(new KirigamiGlobalToolBar(this)) {}
    KirigamiGlobalToolBar* globalToolBar() const { return m_globalToolBar; }
    QQuickItem* initialPage() const { return m_initialPage; }
    void setInitialPage(QQuickItem* p) {
        if (m_initialPage != p) {
            m_initialPage = p;
            Q_EMIT initialPageChanged();
        }
    }
Q_SIGNALS:
    void initialPageChanged();
private:
    KirigamiGlobalToolBar* m_globalToolBar = nullptr;
    QQuickItem* m_initialPage = nullptr;
};

class KirigamiApplicationHeaderStyle : public QObject {
    Q_OBJECT
public:
    enum Status {
        Auto = 0,
        Breadcrumb = 1,
        Titles = 2,
        TabBar = 3,
        ToolBar = 4,
        None = 5
    };
    Q_ENUM(Status)
    explicit KirigamiApplicationHeaderStyle(QObject* parent = nullptr) : QObject(parent) {}
};

class KirigamiMessageType : public QObject {
    Q_OBJECT
public:
    enum Type {
        Information = 0,
        Positive = 1,
        Warning = 2,
        Error = 3
    };
    Q_ENUM(Type)
    explicit KirigamiMessageType(QObject* parent = nullptr) : QObject(parent) {}
};

class DuckyScriptSyntaxHighlighter : public QSyntaxHighlighter {
public:
    explicit DuckyScriptSyntaxHighlighter(QTextDocument* doc, bool dark)
        : QSyntaxHighlighter(doc), m_dark(dark) {}

    void setDark(bool dark) {
        if (m_dark != dark) {
            m_dark = dark;
            rehighlight();
        }
    }

protected:
    void highlightBlock(const QString& text) override {
        const QString trimmed = text.trimmed();
        if (trimmed.startsWith(QStringLiteral("REM "))
            || trimmed == QStringLiteral("REM")
            || trimmed.startsWith(QStringLiteral("//"))
            || trimmed.startsWith(QStringLiteral("#"))) {
            QTextCharFormat commentFmt;
            commentFmt.setForeground(m_dark ? QColor(0x7f, 0x8c, 0x8d) : QColor(0x70, 0x7d, 0x88));
            commentFmt.setFontItalic(true);
            setFormat(0, text.length(), commentFmt);
            return;
        }

        QTextCharFormat kwFmt;
        kwFmt.setForeground(m_dark ? QColor(0x3d, 0xae, 0xe9) : QColor(0x00, 0x66, 0xb8));
        kwFmt.setFontWeight(QFont::Bold);

        static const QRegularExpression kwRe(QStringLiteral(
            "\\b(STRING|STRINGLN|DELAY|DEFAULTDELAY|DEFAULTCHARDELAY|CHARJITTER|KEYDOWN|KEYUP|"
            "WHILE|END_WHILE|IF|ELSE|ELSE_IF|END_IF|VAR|DEFINE|FUNCTION|END_FUNCTION|RETURN|"
            "BREAK|CONTINUE|SWCC|SWCF|SWCR|OLED_CLEAR|OLED_PRINT|OLED_CPRINT|OLED_LPRINT|"
            "OLED_UPDATE|OLED_CURSOR|OLED_LINE|OLED_RECT|OLED_CIRCLE|BCLR|GOTO_PROFILE|"
            "NEXT_PROFILE|PREV_PROFILE|DP_SLEEP|EMUK|LCR|MMOV|MCLK|MSCL|"
            "GUI|WINDOWS|CONTROL|CTRL|ALT|SHIFT|ENTER|SPACE|TAB|ESC|ESCAPE|BACKSPACE|DELETE|UP|DOWN|LEFT|RIGHT)\\b"
        ));
        auto it = kwRe.globalMatch(text);
        while (it.hasNext()) {
            auto m = it.next();
            setFormat(m.capturedStart(), m.capturedLength(), kwFmt);
        }

        QTextCharFormat varFmt;
        varFmt.setForeground(m_dark ? QColor(0x27, 0xae, 0x60) : QColor(0x16, 0x82, 0x45));
        static const QRegularExpression varRe(QStringLiteral("(\\$_[A-Za-z0-9_]+|\\$[A-Za-z0-9_]+|_GV\\d+)"));
        auto vit = varRe.globalMatch(text);
        while (vit.hasNext()) {
            auto m = vit.next();
            setFormat(m.capturedStart(), m.capturedLength(), varFmt);
        }

        QTextCharFormat numFmt;
        numFmt.setForeground(m_dark ? QColor(0xf6, 0x74, 0x00) : QColor(0xb8, 0x56, 0x00));
        static const QRegularExpression numRe(QStringLiteral("\\b(0x[0-9A-Fa-f]+|\\d+)\\b"));
        auto nit = numRe.globalMatch(text);
        while (nit.hasNext()) {
            auto m = nit.next();
            setFormat(m.capturedStart(), m.capturedLength(), numFmt);
        }
    }

private:
    bool m_dark = true;
};

class KSyntaxRepository : public QObject {
    Q_OBJECT
public:
    enum DefaultTheme {
        DarkTheme = 0,
        LightTheme = 1
    };
    Q_ENUM(DefaultTheme)

    explicit KSyntaxRepository(QObject* parent = nullptr) : QObject(parent) {}

    Q_INVOKABLE QVariant defaultTheme(int themeType) const {
        return QVariant::fromValue(themeType);
    }
};

class KSyntaxHighlighter : public QObject {
    Q_OBJECT
    Q_PROPERTY(QObject* textEdit READ textEdit WRITE setTextEdit NOTIFY textEditChanged FINAL)
    Q_PROPERTY(QString definition READ definition WRITE setDefinition NOTIFY definitionChanged FINAL)
    Q_PROPERTY(QObject* repository READ repository WRITE setRepository NOTIFY repositoryChanged FINAL)
    Q_PROPERTY(QVariant theme READ theme WRITE setTheme NOTIFY themeChanged FINAL)

public:
    explicit KSyntaxHighlighter(QObject* parent = nullptr) : QObject(parent) {}

    QObject* textEdit() const { return m_textEdit; }
    void setTextEdit(QObject* te) {
        if (m_textEdit != te) {
            m_textEdit = te;
            attachHighlighter();
            Q_EMIT textEditChanged();
        }
    }

    QString definition() const { return m_definition; }
    void setDefinition(const QString& def) {
        if (m_definition != def) {
            m_definition = def;
            Q_EMIT definitionChanged();
        }
    }

    QObject* repository() const { return m_repository; }
    void setRepository(QObject* repo) {
        if (m_repository != repo) {
            m_repository = repo;
            Q_EMIT repositoryChanged();
        }
    }

    QVariant theme() const { return m_theme; }
    void setTheme(const QVariant& th) {
        if (m_theme != th) {
            m_theme = th;
            if (m_highlighter) {
                m_highlighter->setDark(th.toInt() == KSyntaxRepository::DarkTheme);
            }
            Q_EMIT themeChanged();
        }
    }

Q_SIGNALS:
    void textEditChanged();
    void definitionChanged();
    void repositoryChanged();
    void themeChanged();

private:
    void attachHighlighter() {
        delete m_highlighter;
        m_highlighter = nullptr;
        if (!m_textEdit) return;
        QVariant docVar = m_textEdit->property("textDocument");
        if (auto* quickDoc = docVar.value<QQuickTextDocument*>()) {
            if (QTextDocument* doc = quickDoc->textDocument()) {
                const bool dark = m_theme.isValid() ? (m_theme.toInt() == KSyntaxRepository::DarkTheme) : !KirigamiTheme::isLightModeRef();
                m_highlighter = new DuckyScriptSyntaxHighlighter(doc, dark);
            }
        }
    }

    QObject* m_textEdit = nullptr;
    QString m_definition;
    QObject* m_repository = nullptr;
    QVariant m_theme;
    DuckyScriptSyntaxHighlighter* m_highlighter = nullptr;
};

inline void register_windows_kirigami_compat() {
    static bool s_registered = false;
    if (s_registered) return;
    s_registered = true;

    qmlRegisterUncreatableType<KirigamiTheme>(
        "org.kde.kirigami", 2, 0, "Theme", QStringLiteral("Attached Property")
    );
    qmlRegisterSingletonType<KirigamiUnits>(
        "org.kde.kirigami", 2, 0, "Units",
        [](QQmlEngine*, QJSEngine*) -> QObject* { return new KirigamiUnits(); }
    );
    qmlRegisterType<KirigamiGlobalToolBar>(
        "org.kde.kirigami", 2, 0, "GlobalToolBar"
    );
    qmlRegisterType<KirigamiPageStack>(
        "org.kde.kirigami", 2, 0, "PageStack"
    );
    qmlRegisterUncreatableType<KirigamiApplicationHeaderStyle>(
        "org.kde.kirigami", 2, 0, "ApplicationHeaderStyle", QStringLiteral("Enum")
    );
    qmlRegisterUncreatableType<KirigamiMessageType>(
        "org.kde.kirigami", 2, 0, "MessageType", QStringLiteral("Enum")
    );

    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/ApplicationWindow.qml")),
        "org.kde.kirigami", 2, 0, "ApplicationWindow"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/Page.qml")),
        "org.kde.kirigami", 2, 0, "Page"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/Heading.qml")),
        "org.kde.kirigami", 2, 0, "Heading"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/Action.qml")),
        "org.kde.kirigami", 2, 0, "Action"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/Icon.qml")),
        "org.kde.kirigami", 2, 0, "Icon"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/InlineMessage.qml")),
        "org.kde.kirigami", 2, 0, "InlineMessage"
    );
    qmlRegisterType(
        QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/kirigami/SearchField.qml")),
        "org.kde.kirigami", 2, 0, "SearchField"
    );

    qmlRegisterSingletonType<KSyntaxRepository>(
        "org.kde.syntaxhighlighting", 1, 0, "Repository",
        [](QQmlEngine*, QJSEngine*) -> QObject* { return new KSyntaxRepository(); }
    );
    qmlRegisterType<KSyntaxHighlighter>(
        "org.kde.syntaxhighlighting", 1, 0, "SyntaxHighlighter"
    );
}

} // namespace duckydeck
