import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import NodeList from '../components/dashboard/NodeList.svelte';

describe('NodeList', () => {
   const mockNodes = [
      {
         id: 'n1',
         name: 'Edge-Node-01',
         ip: '192.168.1.10',
         status: 'online',
         cpu: '12%',
         ram: '2.4GB',
      },
      {
         id: 'n2',
         name: 'Edge-Node-02',
         ip: '192.168.1.11',
         status: 'syncing',
         cpu: '45%',
         ram: '4.1GB',
      },
      {
         id: 'n3',
         name: 'Edge-Node-03',
         ip: '192.168.1.12',
         status: 'offline',
         cpu: '-',
         ram: '-',
      },
   ];

   beforeEach(() => {
      vi.clearAllMocks();
   });

   it('renders all nodes', () => {
      render(NodeList, { props: { nodes: mockNodes } });

      expect(screen.getByText('Edge-Node-01')).toBeInTheDocument();
      expect(screen.getByText('Edge-Node-02')).toBeInTheDocument();
      expect(screen.getByText('Edge-Node-03')).toBeInTheDocument();
   });

   it('displays node IP addresses', () => {
      render(NodeList, { props: { nodes: mockNodes } });

      expect(screen.getByText('192.168.1.10')).toBeInTheDocument();
      expect(screen.getByText('192.168.1.11')).toBeInTheDocument();
      expect(screen.getByText('192.168.1.12')).toBeInTheDocument();
   });

   it('shows correct status badges', () => {
      render(NodeList, { props: { nodes: mockNodes } });

      const onlineBadge = screen.getByText('online');
      const syncingBadge = screen.getByText('syncing');
      const offlineBadge = screen.getByText('offline');

      expect(onlineBadge).toBeInTheDocument();
      expect(syncingBadge).toBeInTheDocument();
      expect(offlineBadge).toBeInTheDocument();

      // Check status colors
      expect(onlineBadge.classList.contains('bg-green-500/20')).toBe(true);
      expect(syncingBadge.classList.contains('bg-blue-500/20')).toBe(true);
      expect(offlineBadge.classList.contains('bg-gray-500/20')).toBe(true);
   });

   it('displays resource usage', () => {
      render(NodeList, { props: { nodes: mockNodes } });

      expect(screen.getByText('12%')).toBeInTheDocument();
      expect(screen.getByText('2.4GB')).toBeInTheDocument();
      expect(screen.getByText('45%')).toBeInTheDocument();
      expect(screen.getByText('4.1GB')).toBeInTheDocument();
   });

   it('handles empty node list', () => {
      render(NodeList, { props: { nodes: [] } });

      expect(screen.queryByRole('row')).not.toBeInTheDocument();
   });

   it('renders table headers', () => {
      render(NodeList, { props: { nodes: mockNodes } });

      expect(screen.getByText('Name')).toBeInTheDocument();
      expect(screen.getByText('Status')).toBeInTheDocument();
      expect(screen.getByText('IP Address')).toBeInTheDocument();
      expect(screen.getByText('CPU')).toBeInTheDocument();
      expect(screen.getByText('RAM')).toBeInTheDocument();
   });
});
