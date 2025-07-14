\NeedsTeXFormat{LaTeX2e}\relax
\ProvidesClass{sikslides}[2018/03/20 Beamer Class for SIK]

\def\aspect{169}
\def\aspectft{43}
\def\aspectsn{169}
\def\aspectst{1610}

% default options
\DeclareOption{43}{\def\aspect{43}}
\DeclareOption{169}{\def\aspect{169}}
\DeclareOption{1610}{\def\aspect{1610}}
\DeclareOption{handout}{\def\hashandout{yes}}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{beamer}}
\ExecuteOptions{xcolor=pdftex,dvipsnames,table}
\ProcessOptions\relax
\ifx\hashandout\undefined
\def\handoutopt{}
\else
\def\handoutopt{,handout}
\fi
\LoadClass[aspectratio=\aspect\handoutopt]{beamer}

\ifx\hashandout\undefined\else
    \setbeameroption{show notes}
\fi


\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage[german]{babel}
\usepackage{graphicx}
\usepackage{multirow}
\usepackage{multicol}
\usepackage[squaren,binary,thinqspace,textstyle]{SIunits}
\usepackage{textpos}
\DeclareGraphicsExtensions{.pdf}
\usepackage{amsmath}
\usepackage{amsfonts}
\usepackage{xfrac}
\usepackage{graphicx}
\usepackage{listings}
\usepackage{algorithm2e}
\usepackage{sansmath}
\usepackage{ulem} 
\usepackage{lmodern} 

\usepackage{include/beamerthemesikfai}

%\usefonttheme[onlymath]{serif}

%\usepackage[familydefault]{Rosario}
%\renewcommand*\familydefault{\rmdefault}
%\usepackage{nimbusmononarrow}

\newcommand{\titleframe}{
    \begin{frame}[plain,noframenumbering]
        \titlepage
    \end{frame}
}

\newcommand{\sectionframe}[2][]{
    \def\temp{#1}
    \ifx\temp\empty
    \section{#2}
    \else
    \section{#1}
    \fi
    \addtocounter{framenumber}{1}
    \begin{frame}[noframenumbering]
        \frametitle{\ }
        \begin{center}
            \Huge{#2}
        \end{center}
    \end{frame}
    \addtocounter{framenumber}{-1}
}


\institute[Uni Augsburg]{Lehrstuhl für Embedded Systems\\
    Institut für Informatik\\
Universität Augsburg}
\subtitle{Seminar}
