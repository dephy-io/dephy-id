// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'DePHY Wiki',
  tagline: 'The best framework to build DePIN Apps',
  favicon: 'img/favicon.svg',

  // Set the production url of your site here
  url: 'https://dephy.io/',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'dephy-io', // Usually your GitHub org/user name.
  projectName: 'website', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'zh-cn'],
    localeConfigs: {
      'en': {
        label: 'English',
      },
      'zh-cn': {
        label: '简体中文',
      }
    }
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/dephy-io/website',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/dephy-io/website',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({

      colorMode:{
        defaultMode: 'light',
      },
      // Replace with your project's social card
      image: 'img/dephy.svg',
      navbar: {
        title: 'DePHY',
        logo: {
          alt: 'DePHY Logo',
          srcDark: "img/logo.svg",
          src: "img/logo_dark.svg",
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'quickstart',
            position: 'left',
            label: 'Quickstart',
          },
          {
            type: 'docSidebar',
            sidebarId: 'guideSidebar',
            position: 'left',
            label: 'Guides',
          },
          // {to: '/blog', label: 'Blog', position: 'left'},
          {
            label: 'Twitter',
            position: 'right',
            href: 'https://twitter.com/dephynetwork',
          },
          {
            label: 'GitHub',
            position: 'right',
            href: 'https://github.com/dephy-io',
          },
          // {
          //   type: 'localeDropdown',
          //   position: 'right',
          // },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Guides',
                to: '/docs/tutorial-basics/tech_background',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discordapp.com/invite/Wbx2BAn2A4',
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/dephynetwork',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/dephy-io',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} DePHY`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
      },
    }),
};

export default config;